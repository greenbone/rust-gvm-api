#!/usr/bin/env python3
"""Resolve rolling direct-OpenVASD feed channels to one-run exact references."""

from __future__ import annotations

import argparse
import json
import re
import subprocess
from collections.abc import Callable
from pathlib import Path
from typing import Any


REPOSITORY_ROOT = Path(__file__).resolve().parents[1]
PINS_PATH = (
    REPOSITORY_ROOT / "tests" / "e2e" / "evidence" / "direct-openvasd-pins.json"
)
FEED_SERVICES = (
    "vulnerability-tests",
    "data-objects",
    "report-formats",
    "gpg-data",
    "notus-data",
)
COMPOSE_ENVIRONMENT = {
    "vulnerability-tests": "GVM_GATEWAY_E2E_VULNERABILITY_TESTS_IMAGE",
    "data-objects": "GVM_GATEWAY_E2E_DATA_OBJECTS_IMAGE",
    "report-formats": "GVM_GATEWAY_E2E_REPORT_FORMATS_IMAGE",
    "gpg-data": "GVM_GATEWAY_E2E_GPG_DATA_IMAGE",
    "notus-data": "GVM_GATEWAY_E2E_NOTUS_DATA_IMAGE",
}
DIGEST_PATTERN = re.compile(r"sha256:[0-9a-f]{64}")
EXACT_REFERENCE_PATTERN = re.compile(r".+:[^/@]+@sha256:[0-9a-f]{64}$")


class FeedResolutionError(RuntimeError):
    """A rolling feed channel cannot be made immutable for this invocation."""


def load_declared_channels(path: Path = PINS_PATH) -> dict[str, str]:
    """Load the closed set of mutable feed channels from the input manifest."""

    try:
        document: Any = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise FeedResolutionError(f"cannot load feed channel manifest {path}: {error}") from error
    channels = document.get("feedDataChannels") if isinstance(document, dict) else None
    if not isinstance(channels, dict) or set(channels) != set(FEED_SERVICES):
        raise FeedResolutionError(
            "feedDataChannels must exactly declare the five rolling feed services"
        )
    result: dict[str, str] = {}
    for name in FEED_SERVICES:
        channel = channels.get(name)
        if (
            not isinstance(channel, str)
            or "@" in channel
            or channel.rfind(":") <= channel.rfind("/")
        ):
            raise FeedResolutionError(
                f"rolling feed {name!r} must declare an explicit mutable channel tag"
            )
        result[name] = channel
    return result


def inspect_channel(channel: str) -> str:
    """Return the registry digest advertised for one channel without pulling it."""

    try:
        result = subprocess.run(
            ["docker", "buildx", "imagetools", "inspect", channel],
            check=False,
            text=True,
            capture_output=True,
            timeout=60,
        )
    except FileNotFoundError as error:
        raise FeedResolutionError(
            "Docker CLI with buildx is required to resolve rolling feed channels"
        ) from error
    except subprocess.TimeoutExpired as error:
        raise FeedResolutionError(f"registry lookup timed out for {channel}") from error
    except OSError as error:
        raise FeedResolutionError(
            f"registry lookup could not start for {channel}: {error}"
        ) from error
    if result.returncode != 0:
        detail = result.stderr.strip() or result.stdout.strip() or "registry lookup failed"
        raise FeedResolutionError(detail)
    digests = {
        match.group(1)
        for line in result.stdout.splitlines()
        if (match := re.fullmatch(r"\s*Digest:\s*(sha256:[0-9a-f]{64})\s*", line))
    }
    if len(digests) != 1:
        raise FeedResolutionError(
            f"registry lookup for {channel} did not return one unambiguous manifest digest"
        )
    return digests.pop()


def resolved_document(
    channels: dict[str, str],
    resolver: Callable[[str], str] | None = None,
) -> dict[str, Any]:
    """Resolve every declared channel exactly once into a closed evidence document."""

    resolver = resolver or inspect_channel
    resolved: dict[str, str] = {}
    for name in FEED_SERVICES:
        channel = channels[name]
        try:
            digest = resolver(channel)
        except FeedResolutionError as error:
            raise FeedResolutionError(
                f"cannot resolve rolling feed {name!r} channel {channel!r}: {error}"
            ) from error
        if not isinstance(digest, str) or not DIGEST_PATTERN.fullmatch(digest):
            raise FeedResolutionError(
                f"resolver returned an invalid digest for rolling feed {name!r}: {digest!r}"
            )
        resolved[name] = f"{channel}@{digest}"
    return {
        "schemaVersion": 1,
        "status": "resolved",
        "declaredChannels": channels,
        "resolvedImages": resolved,
        "failure": None,
    }


def failed_document(channels: dict[str, str], error: FeedResolutionError) -> dict[str, Any]:
    """Retain a machine-readable setup/input failure without inventing resolutions."""

    return {
        "schemaVersion": 1,
        "status": "failed",
        "declaredChannels": channels,
        "resolvedImages": {},
        "failure": str(error),
    }


def validate_resolution(
    document: Any, expected_channels: dict[str, str]
) -> dict[str, str]:
    """Validate a successful resolution and return its exact image references."""

    expected_keys = {
        "schemaVersion",
        "status",
        "declaredChannels",
        "resolvedImages",
        "failure",
    }
    if not isinstance(document, dict) or set(document) != expected_keys:
        raise FeedResolutionError("feed resolution has incomplete or unknown fields")
    if (
        document["schemaVersion"] != 1
        or document["status"] != "resolved"
        or document["failure"] is not None
        or document["declaredChannels"] != expected_channels
    ):
        raise FeedResolutionError("feed resolution does not match the declared channels")
    images = document["resolvedImages"]
    if not isinstance(images, dict) or set(images) != set(FEED_SERVICES):
        raise FeedResolutionError("feed resolution does not cover every rolling feed")
    validated: dict[str, str] = {}
    for name in FEED_SERVICES:
        reference = images.get(name)
        channel = expected_channels[name]
        if (
            not isinstance(reference, str)
            or not EXACT_REFERENCE_PATTERN.fullmatch(reference)
            or not reference.startswith(f"{channel}@")
        ):
            raise FeedResolutionError(
                f"feed resolution for {name!r} is not an exact reference for {channel!r}"
            )
        validated[name] = reference
    return validated


def load_resolution(
    path: Path, expected_channels: dict[str, str]
) -> dict[str, str]:
    """Load and validate a successful resolution document from disk."""

    try:
        document = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise FeedResolutionError(f"cannot load feed resolution {path}: {error}") from error
    return validate_resolution(document, expected_channels)


def write_json(path: Path, document: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(document, indent=2) + "\n", encoding="utf-8")


def write_compose_environment(path: Path, images: dict[str, str]) -> None:
    """Write only fixed-name, strictly validated values for the Compose launcher."""

    path.parent.mkdir(parents=True, exist_ok=True)
    lines = [f"{COMPOSE_ENVIRONMENT[name]}={images[name]}" for name in FEED_SERVICES]
    path.write_text("\n".join(lines) + "\n", encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--pins", type=Path, default=PINS_PATH)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--compose-env", type=Path, required=True)
    args = parser.parse_args()

    channels = load_declared_channels(args.pins)
    args.compose_env.unlink(missing_ok=True)
    try:
        document = resolved_document(channels)
    except FeedResolutionError as error:
        write_json(args.output, failed_document(channels, error))
        raise
    images = validate_resolution(document, channels)
    write_json(args.output, document)
    write_compose_environment(args.compose_env, images)
    print(f"resolved {len(images)} rolling feed channels in {args.output}")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except FeedResolutionError as error:
        raise SystemExit(f"rolling feed image resolution failed: {error}") from error
