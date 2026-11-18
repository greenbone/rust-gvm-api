"""Shared comparison identity for direct-OpenVASD promotion evidence."""

from __future__ import annotations

import hashlib
import json
import re
from typing import Any


EXACT_FEED_REFERENCE = re.compile(r"(?P<channel>.+:[^/@]+)@sha256:[0-9a-f]{64}$")
LEGACY_FEED_REVISION = re.compile(r"sha256:.+")


class ProvenanceFingerprintError(ValueError):
    """Direct-lane provenance cannot safely enter one comparison window."""


def canonical_sha256(value: Any) -> str:
    encoded = json.dumps(value, sort_keys=True, separators=(",", ":")).encode("utf-8")
    return hashlib.sha256(encoded).hexdigest()


def normalized_feed_data(feed_data: Any) -> list[dict[str, str]]:
    """Compare new feed evidence by channel while preserving legacy ledger hashes."""

    if not isinstance(feed_data, list) or not feed_data:
        raise ProvenanceFingerprintError("finalized provenance feedData is incomplete")
    rows: list[dict[str, str]] = []
    modes: set[str] = set()
    names: set[str] = set()
    for item in feed_data:
        if not isinstance(item, dict) or set(item) != {"name", "revision"}:
            raise ProvenanceFingerprintError("finalized provenance feedData row is invalid")
        name = item.get("name")
        revision = item.get("revision")
        if (
            not isinstance(name, str)
            or not name
            or name in names
            or not isinstance(revision, str)
        ):
            raise ProvenanceFingerprintError(
                "finalized provenance feedData is invalid or duplicated"
            )
        names.add(name)
        if exact := EXACT_FEED_REFERENCE.fullmatch(revision):
            modes.add("channel")
            rows.append({"name": name, "revision": exact.group("channel")})
        elif LEGACY_FEED_REVISION.fullmatch(revision):
            modes.add("legacy")
            rows.append({"name": name, "revision": revision})
        else:
            raise ProvenanceFingerprintError(
                f"finalized provenance feedData revision for {name!r} is not exact"
            )
    if len(modes) != 1:
        raise ProvenanceFingerprintError(
            "finalized provenance cannot mix legacy and channel-resolved feedData"
        )
    return rows


def provenance_fingerprint(provenance: dict[str, Any]) -> str:
    """Hash repository-controlled inputs, ignoring runner and rolling feed digests."""

    required = {
        "schemaVersion",
        "gvmd",
        "scanner",
        "feedData",
        "fixtures",
        "apiContract",
        "composeSha256",
        "runner",
    }
    if not isinstance(provenance, dict):
        raise ProvenanceFingerprintError("finalized provenance must be an object")
    if provenance.get("schemaVersion") != 1 or not required <= provenance.keys():
        raise ProvenanceFingerprintError("finalized provenance is incomplete")
    fixtures = provenance.get("fixtures")
    if not isinstance(fixtures, list):
        raise ProvenanceFingerprintError("finalized provenance fixtures are incomplete")
    revisions = {
        item.get("name"): item.get("revision")
        for item in fixtures
        if isinstance(item, dict)
    }
    identities = {
        "GVM_GATEWAY_E2E_TARGET_HOST": "e2e-target",
        "scanner-backend-identity": "openvasd-backend",
        "synthetic-target-identity": "e2e-target",
    }
    for name, expected in identities.items():
        if revisions.get(name) != expected:
            raise ProvenanceFingerprintError(
                f"finalized provenance identity {name!r} is invalid"
            )
    exact_inputs = {
        "tests/e2e/compose.direct-openvasd.yaml",
        "synthetic-target-image",
        "tests/e2e/evidence/direct-openvasd-pins.json",
        "gvm-openapi-emulator/examples/openvasd/swagger.json",
        "gvm-openapi-emulator/examples/openvasd",
        "tests/e2e-harness",
    }
    for name in exact_inputs:
        if not re.fullmatch(r"sha256:[0-9a-f]{64}", str(revisions.get(name))):
            raise ProvenanceFingerprintError(
                f"finalized provenance input {name!r} is not exact"
            )

    comparable = {name: provenance[name] for name in sorted(required - {"runner"})}
    comparable["feedData"] = normalized_feed_data(provenance["feedData"])
    return canonical_sha256(comparable)
