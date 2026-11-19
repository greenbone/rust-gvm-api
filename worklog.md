# GMP feature specification worklog

## 2026-11-19

- Confirmed clean `b02/gmp-feature-spec` checkout; read AGENTS.md, canonical REST design/test specification and curated/generated OpenAPI organization before editing.
- Reviewed paired issue evidence for rust-gvm #847–857 / API #727–736 and actual typed implementations at rust-gvm commit `2ee57897e2121e600e278247c61635100d00d85a` in `/workspace/repos/rust-gvm-mock-docs`.
- Scope: documentation-only planned contract supplement. Do not change shipped OpenAPI, runtime, dependency pin, surface ledger or claim downstream issues complete. Scanner #850 has no REST write counterpart.
- Added `spec/rest-api/planned-gmp-features.md`: all ten paired gaps, proposed field/route semantics, omission/default rules, capability/error/confidentiality requirements, acceptance criteria and unresolved decisions. Linked from README, docs index, openspec, test-spec and upstream disposition narrative.
- Source review caught two important constraints: target writes accept only `manual`/`active_directory` (reads remain open strings), and report scan-config metadata lives in structured report tasks, not the generic report/task projection currently consumed by the gateway. Aggregate vulnerability/closed-CVE models do not yet supply the NVT discriminator.
- Validation passed: `python3 scripts/check-docs.py` (121 implemented paths / 199 operations, relative links intact); `git diff --check`; focused Python assertions covering ten API/upstream pairs and no YAML/Rust/dependency/TSV modifications.
- Attempted required `cargo fmt --all -- --check`, including retry after sourcing `/workspace/.dev-env`: unavailable (`cargo: not found`). No Rust/clippy/test suite run for this documentation-only change. No served OpenAPI, runtime or dependency changes.
- Remaining decisions are explicit in the supplement: sensitive-operation route/problem/size policy, structured-schedule response compatibility/versioning, report typed read strategy and endpoint coverage, immutable dependency adoption and live backend qualification. No push or PR performed.
- Parent verification: workspace-local toolchain successfully ran `cargo fmt --all -- --check`; documentation validation independently passes (121 paths / 199 operations). Corrected partial AD modify semantics and pinned published upstream b6f33da1224aa4950f39dc3db0e6d5607e0e0f89.
- Published docs/design PR https://github.com/greenbone/rust-gvm-api/pull/740; served OpenAPI and runtime remain unchanged. All ten downstream issues remain open.

## PR #740 manual stack verification

- Parent explicitly requested adding only `workflow_dispatch` to CI so downstream stack branches can receive exact-head CI verification despite existing main/devel PR filters. Retain filters, classification, job definitions and gates. Existing classifier intentionally treats manual runs as code-affecting; direct workflow_dispatch verification produced docs_only=false, ci_required=true and e2e_required=true. Post-edit fmt and strict workspace clippy checks pass.
- bb6c817 exact-head CI (44590747927), Build (44590748955) and Security (44590747619) passed. E2E 44590747525 exhausted all three registry retries in both lanes; direct read-only probes of registry `/v2/`, exact pg-gvm digest, and redis reference independently returned nginx 502 without Retry-After. No source-backed image/pin-preserving workaround found for the upstream service outage; pins and checks remain unchanged.

## PR #740 registry setup recovery

- Attempt 2 of E2E run 44519075684 again failed on community registry HTTP 502: required REST redis pull and advisory shadow pg-gvm immutable-manifest preflight (`/workspace/740-e2e-attempt2.log`).
- Read AGENTS.md, spec/rest-api/test-spec.md E2E/CI contract, and tests/e2e/evidence/README.md fail-closed immutable-input policy. Preserve classification, gates, exact pins, rolling-feed resolution policy and image selection.
- Add shared three-attempt recovery only for explicit HTTP 502/503/504, with 2/4-second backoff and immediate permanent-error failure. Manifest checks keep identical digest-only argv and 60-second per-attempt timeout. REST pulls run separately with a 600-second per-attempt timeout before a single startup with --pull never (including pull_policy: always services).
- Verification passed: four new registry regression tests (including actual CI Bash + fake Docker transient/permanent/exhaustion/startup-failure scenarios); 70 existing E2E evidence tests; 23 enterprise evidence tests; evidence validator (45 inventoried Rust tests); docs checker (121 paths / 199 operations); Bash syntax; cargo fmt --all -- --check; git diff --check. New regressions are wired into e2e-evidence-check. `make` is unavailable locally, so its Python commands ran directly.
- Initial clippy attempt failed before crate checks because linker `cc` was missing. Follow-up verification using `/workspace/api-agent-env` and isolated `/workspace/.cache/api-740-target` passed strict `cargo clippy --workspace --all-targets --all-features -- -D warnings` (exit 0); fmt check, full `make e2e-evidence-check` and docs checker also passed. No Rust changes. Parent authorized pushing the existing PR740 branch and observing exact-head CI/E2E; no merge or separate PR.
