#!/usr/bin/env python3
"""Regression tests: registry recovery cannot weaken fail-closed E2E inputs."""

import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import call, patch

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "scripts"))
import registry_retry as RETRY

spec = importlib.util.spec_from_file_location(
    "pins_validator", ROOT / "scripts/validate-direct-openvasd-pins.py"
)
PINS = importlib.util.module_from_spec(spec)
spec.loader.exec_module(PINS)


class RegistryRetryTests(unittest.TestCase):
    def test_exact_digest_request_recovers_from_transient_statuses(self):
        """No tag resolution or alternate image may replace the retained manifest."""
        image = "registry.example:5443/team/pg:stable@sha256:" + "a" * 64
        argv = ["docker", "manifest", "inspect", PINS.digest_reference(image)]
        for status in ("502 Bad Gateway", "503 Service Unavailable", "504 Gateway Timeout"):
            with self.subTest(status=status), patch.object(
                RETRY.subprocess, "run", side_effect=[
                    subprocess.CompletedProcess(argv, 1, "", "received unexpected HTTP status: " + status),
                    subprocess.CompletedProcess(argv, 0, "{}", ""),
                ]
            ) as run, patch.object(RETRY.time, "sleep") as sleep:
                PINS.inspect_manifest(image)
                self.assertEqual(run.call_args_list, [
                    call(argv, check=False, text=True, capture_output=True, timeout=60)
                ] * 2)
                sleep.assert_called_once_with(2)

    def test_exhaustion_names_pin_and_stops_downstream_checks(self):
        """Three failures remain setup failure, with no hidden fourth attempt."""
        image = "registry.example/pg:stable@sha256:" + "b" * 64
        with patch.object(RETRY.subprocess, "run", return_value=
            subprocess.CompletedProcess([], 1, "", "HTTP status: 502 Bad Gateway")
        ) as run, patch.object(RETRY.time, "sleep") as sleep:
            with self.assertRaisesRegex(PINS.RuntimeImageManifestError, "unavailable runtime image 'pg'.*502"):
                PINS.validate_runtime_images({"pg": image, "z-next": image + "bad"})
            self.assertEqual(run.call_count, 3)
            self.assertEqual(sleep.call_args_list, [call(2), call(4)])
            self.assertTrue(all(c.args[0][-1] == PINS.digest_reference(image) for c in run.call_args_list))

    def test_permanent_unknown_and_launch_errors_are_not_retried(self):
        """Auth, missing manifests and non-HTTP errors fail on the first attempt."""
        for detail in ("unauthorized", "manifest unknown", "HTTP status: 404 Not Found",
                       "invalid reference format", "connection refused", "no space left on device",
                       "HTTP status: 502 Bad Gateway\nmanifest unknown"):
            with self.subTest(detail=detail), patch.object(RETRY.subprocess, "run", return_value=
                subprocess.CompletedProcess([], 1, "", detail)
            ) as run, patch.object(RETRY.time, "sleep") as sleep:
                with self.assertRaises(PINS.RuntimeImageManifestError):
                    PINS.inspect_manifest("registry.example/pg:stable@sha256:" + "b" * 64)
                run.assert_called_once()
                sleep.assert_not_called()
        for error in (FileNotFoundError("docker"), subprocess.TimeoutExpired(["docker"], 60)):
            with self.subTest(error=error), patch.object(RETRY.subprocess, "run", side_effect=error) as run:
                with self.assertRaises(PINS.RuntimeImageManifestError):
                    PINS.inspect_manifest("registry.example/pg:stable@sha256:" + "b" * 64)
                run.assert_called_once()

    def test_workflow_shell_pulls_before_one_startup_and_stops_after_failed_pull(self):
        """Execute actual CI shell with mocked Docker; no Docker/registry needed."""
        workflow = (ROOT / ".github/workflows/nightly-e2e.yml").read_text()
        block = workflow.split("      - name: Start compose-backed runtime\n        run: |\n", 1)[1].split("      - name:", 1)[0]
        shell = "\n".join(line[10:] for line in block.splitlines())
        with tempfile.TemporaryDirectory(dir=ROOT) as directory:
            path = Path(directory)
            docker = path / "docker"
            docker.write_text("""#!/usr/bin/env python3
import json, os, pathlib, sys
root = pathlib.Path(os.environ["MOCK_REGISTRY_DIR"])
args = sys.argv[1:]
with (root / "calls").open("a") as stream:
    stream.write(json.dumps(args) + "\\n")
if args == ["compose", "version"]:
    sys.exit(0)
if "pull" in args:
    state = root / "attempt"
    count = int(state.read_text()) + 1 if state.exists() else 1
    state.write_text(str(count))
    mode = os.environ["MOCK_REGISTRY_MODE"]
    if mode == "permanent":
        sys.exit("manifest unknown")
    if mode == "exhaust" or (mode == "transient" and count == 1):
        sys.exit("unexpected status from HEAD request: 502 Bad Gateway")
if "up" in args and os.environ["MOCK_REGISTRY_MODE"] == "startup":
    sys.exit("HTTP status: 502 Bad Gateway")
""")
            docker.chmod(0o755)
            for mode, pulls, starts, success in (
                ("transient", 2, 1, True), ("permanent", 1, 0, False),
                ("exhaust", 3, 0, False), ("startup", 1, 1, False),
            ):
                with self.subTest(mode=mode):
                    for name in ("calls", "attempt"):
                        (path / name).unlink(missing_ok=True)
                    result = subprocess.run(["bash", "-euo", "pipefail", "-c", shell],
                        cwd=ROOT, capture_output=True, text=True, timeout=20,
                        env={**os.environ, "PATH": str(path) + os.pathsep + os.environ["PATH"],
                             "MOCK_REGISTRY_DIR": directory, "MOCK_REGISTRY_MODE": mode})
                    self.assertEqual(result.returncode == 0, success, result.stderr)
                    calls = [json.loads(line) for line in (path / "calls").read_text().splitlines()]
                    pull_calls = [c for c in calls if "pull" in c]
                    up_calls = [c for c in calls if "up" in c]
                    self.assertEqual(len(pull_calls), pulls)
                    self.assertTrue(all(c == pull_calls[0] for c in pull_calls))
                    self.assertIn("--ignore-buildable", pull_calls[0])
                    self.assertEqual(len(up_calls), starts)
                    if starts:
                        self.assertEqual(up_calls[0][-5:], ["up", "-d", "--build", "--pull", "never"])
                        self.assertLess(calls.index(pull_calls[0]), calls.index(up_calls[0]))


if __name__ == "__main__":
    unittest.main()
