"""The design route inventory must describe only implemented stacked routes."""

from __future__ import annotations

import importlib.util
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location("check_docs", ROOT / "scripts/check-docs.py")
assert SPEC is not None and SPEC.loader is not None
CHECK_DOCS = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(CHECK_DOCS)

SAVED = "/credentials/{id}/actions/verify-active-directory"
INLINE = "/active-directory/credential-verifications"
SEARCH = "/active-directory/object-searches"


class CredentialsInventoryTests(unittest.TestCase):
    def check(self, documented: list[str], implemented: list[str]) -> list[str]:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            directory = root / "spec/rest-api"
            directory.mkdir(parents=True)
            rows = "\n".join(
                f"| `POST` | `/gvm/public/v1{path}` | Operation |" for path in documented
            )
            (directory / "openspec.md").write_text(f"#### Credentials\n{rows}\n#### Next\n")
            with patch.object(CHECK_DOCS, "REPOSITORY_ROOT", root), patch.object(
                CHECK_DOCS, "curated_routes", return_value={path: {"POST"} for path in implemented}
            ):
                return CHECK_DOCS.check_credentials_inventory()

    def test_verification_slice_cannot_advertise_future_search_or_omit_implemented_routes(self):
        # This is the exact #747 review: a future route replaced both live ones.
        failures = self.check([SEARCH], [SAVED, INLINE])
        self.assertIn(f"OpenSpec Credentials has unimplemented route {SEARCH}", failures)
        for path in [SAVED, INLINE]:
            self.assertIn(f"OpenSpec Credentials is missing route {path}", failures)

    def test_verification_slice_lists_both_implemented_operations(self):
        self.assertEqual(self.check([SAVED, INLINE], [SAVED, INLINE]), [])

    def test_search_route_is_introduced_only_with_its_runtime_contract(self):
        # The next slice must add its row together with the actual OpenAPI path.
        self.assertIn(
            f"OpenSpec Credentials is missing route {SEARCH}",
            self.check([SAVED, INLINE], [SAVED, INLINE, SEARCH]),
        )
        self.assertEqual(self.check([SAVED, INLINE, SEARCH], [SAVED, INLINE, SEARCH]), [])


if __name__ == "__main__":
    unittest.main()
