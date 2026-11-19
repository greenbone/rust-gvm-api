#!/usr/bin/env python3
"""Bound registry reads to three attempts, retrying only explicit HTTP 502/503/504 failures."""

from __future__ import annotations

import re
import subprocess
import sys
import time
from collections.abc import Sequence


# Do not retry a batch containing a known permanent error, even if another
# parallel pull reports a transient status. Unknown/non-HTTP failures fail closed.
PERMANENT_ERROR = re.compile(
    r"unauthorized|authentication required|access denied|pull access denied|"
    r"manifest unknown|name unknown|no such manifest|not found|invalid reference|"
    r"(?:http(?: status)?|status(?: code)?)\s*[:=]?\s*(?:400|401|403|404)\b",
    re.IGNORECASE,
)
TRANSIENT_HTTP_ERROR = re.compile(
    r"(?:http(?: status)?|status(?: code)?(?: from [^\n]*?)?)\s*[:=]?\s*(?:502|503|504)\b|"
    r"\b(?:502 Bad Gateway|503 Service Unavailable|504 Gateway Timeout)\b",
    re.IGNORECASE,
)
BACKOFF_SECONDS = (2, 4)


def run_registry_command(
    command: Sequence[str], *, timeout: int
) -> subprocess.CompletedProcess[str]:
    """Repeat the identical read/pull argv, never a container startup operation.

    Each attempt has a fixed timeout. Launch errors and timeouts are not explicit
    HTTP failures and propagate immediately; callers retain their diagnostics.
    """

    for attempt in range(len(BACKOFF_SECONDS) + 1):
        result = subprocess.run(
            command, check=False, text=True, capture_output=True, timeout=timeout
        )
        if result.returncode == 0:
            return result
        detail = result.stderr + "\n" + result.stdout
        if (
            attempt == len(BACKOFF_SECONDS)
            or PERMANENT_ERROR.search(detail)
            or not TRANSIENT_HTTP_ERROR.search(detail)
        ):
            return result
        delay = BACKOFF_SECONDS[attempt]
        print(
            f"Registry HTTP failure (attempt {attempt + 1}/3); "
            f"retrying identical request in {delay}s:\n{detail.strip()}",
            file=sys.stderr,
        )
        time.sleep(delay)
    raise AssertionError("unreachable registry retry state")


def main() -> int:
    """CI pull entry point: a maximum 30 minutes plus six seconds, no startup retry."""

    command = sys.argv[1:]
    if not command:
        raise SystemExit("Usage: registry_retry.py <registry-read-or-pull-command...>")
    try:
        result = run_registry_command(command, timeout=600)
    except (OSError, subprocess.TimeoutExpired) as error:
        print(f"Registry command failed: {error}", file=sys.stderr)
        return 1
    print(result.stdout, end="")
    print(result.stderr, end="", file=sys.stderr)
    return result.returncode


if __name__ == "__main__":
    raise SystemExit(main())
