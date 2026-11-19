# AD API implementation stack

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
- Adopted upstream immutable feature stack b6f33da; reviewed 270-method typed facade (three newly blocked REST adopters). Building isolated target; native compiler provisioned inside workspace only.
- Prerequisite commit b6b901b compiles all workspace features; SHA also recorded in /workspace/api-agent-baseline.txt for parent coordination (no sessions-send tool available).
- #730 runtime maps structured schedules/delays with legacy writes and explicit documented Tech Preview response break.
- #730 acceptance: 12 REST agent tests passed; strict all-target/all-feature clippy passed before token slice.
- #731 implemented typed backend-only token issuance, no-store REST, confidential domain wrapper and sanitized status-aware failure mapping including distinct 503.

## #731 independent-head contract correction

- Exact-head CI exposed Aide inferring a request body from the rejection-only Bytes extractor. Move the existing later-slice suppression into inventory-token issuance itself, preserving runtime body rejection and the curated no-body contract.
- Add an explicit generated-OpenAPI regression asserting this operation has no requestBody.
- #732 projects all four typed optional OS metadata values to domain and REST list/detail schemas without inference or enum narrowing.
- #731 acceptance: three typed issuance/discovery/error/redaction tests pass. #732 typed empty/absence/future-value parser and REST list/detail tests pass; strict clippy passes.
- #733 optional anonymized query now reaches typed support-bundle request independently of encryption.
- Full workspace test attempt hit native compressed-debug linker incompatibility (lld lacks zlib); isolated CFLAGS now disable native compressed/debug sections, resolving it. No repository toolchain files changed.
- Broad gateway contract regression surfaced unrelated upstream report application shape change (expected 3 keys, now 2) in report_drill_down_and_operating_system_routes_expose_typed_contracts. Parent report slices own that projection; do not weaken the contract in agent scope.
- Generated/curated schema parity initially caught union mismatch; changed curated legacy/structured schedule input union to anyOf, matching generated serde-untagged schema. Added explicit generated Cache-Control no-store header.
- Final evidence: changed-layer all-feature suites passed: app 72, domain 64, gvmd unit 90, architecture 22, typed adapter integration 120, REST 201 (669 total), plus all five HTTP agent tests. Latest privacy subset passed 3 tests (nine combinations and unsupported backend); latest REST agent subset passed 15. The new explicit HTTP 503 test passes.
- Generated/curated OpenAPI parity now passes end-to-end after removing the inferred binary request body from the no-body issuance route and defining problem+json content for 500/503. Strict all-workspace/all-target/all-feature clippy and fmt check passed on final code.
- Earlier whole-workspace attempt still has an unrelated report application fixture contract mismatch (3 keys versus 2); parent confirms combined report/#741 workspace suite passes. No live gvmd qualification claimed.
- Final slice includes acceptance-hardening followups for preceding agent slices (non-null delay validation, group delay lifecycle assertions, no-body/cache/error OpenAPI fidelity). Earlier published slice commits are unchanged.

## AD implementation

- Created b02/api-ad-impl at PR740 9a5664f.
- Adopted parent-owned immutable dependency baseline b6b901b (local cherry-pick a0b2e1b).
- Implementing #727 credential writes/read metadata + saved/inline verification; #728 search and #729 provenance isolated in parallel file scopes.
- Native compiler inherited from /workspace/api-agent-env; target /workspace/.cache/api-ad-target. Initial compile exposed REST DTO omissions, fixed; focused test build underway.
- #727 includes credential read/write AD metadata, validation, saved/inline verification, sanitized typed failures and discovery. Credential REST tests: 15 passed. Typed verification fixture: passed (scripted only, not LDAP). OpenAPI enum-inventory assertions were updated after initial failures.
- #728 bounded read-only search implemented with optional envelope/count/expansion preservation and no-store; no write fallback. Shared strict all-target/all-feature clippy passed; complete REST suite and HTTP AD credential fixtures are running after final tests/schema refinements.

- #729 target provenance implemented with closed writes, open optional reads, omission/default preservation and server-owned cloning/filtering. Independent focused tests passed: HTTP 2, REST 10, typed conversion 1.
- Parent integration follow-up: added missing human API-reference rows and behavior notes in issue-specific #727/#728 docs corrections. AD worktree curated contract has 124 paths/202 operations; combined parent stack has an additional non-AD route (125/203), so preserve the parent inventory increment while cherry-picking these docs corrections.

# Report implementation worklog

- Read repository rules, curated report/result contracts and #734–736/#742. Rooted at PR740 9a5664f.
- #734: preserve typed optional counts/threat and server severity. Dependency baseline pending.

- #734 slice-boundary correction: explicit u32 schema bounds now live with application counts, not the later reference slice. Added generated schema regression; preserved curated limits.
- #735: open JSON type on full/summary result NVTs; typed note/override references no longer mislabel discriminator as family. Aggregate sources retain absence.
- #735 acceptance review: typed classic/nonclassic/future/empty/absent discriminator projections qualified; description comments corrected.

- #736: detail/JSON-export scope selected; typed structured report snapshot supplies task.scanConfig; generic list cannot supply it without upstream generic model extension.
- #736 compatibility review: unsupported snapshot omits enrichment only; real backend errors propagate. Focused older-backend/error regressions pass.
- #736 absent snapshot task names remain absent, with targeted regression coverage.

- #742: raw typed refs mapped on ordinary result/catalog paths. KEV-specific typed/source gaps isolated; producer and library issue drafts prepared for parent filing; no public issue creation or pushes.

- Adopted parent baseline b6b901b as cherry-pick 1a5f802 (worklog conflict resolved locally); no independent dependency upgrade.
- Verification: cargo test --workspace passed, including generated/curated contract parity and typed application/NVT/snapshot/ref regressions. Required Clippy --workspace --all-targets --all-features -- -D warnings passed; fmt/check and diff/check passed. Existing manifest-only unused-dependency warnings remain.
- Native compiler reused from parent workspace-local cache with CFLAGS --sysroot=/workspace/.cache/api-agent-native -g0; the initial native build failed without cc and then with compressed native debug sections unsupported by rust-lld. Corrected environment /workspace/report-test-env; successful run /workspace/report-tests2.log, Clippy /workspace/report-clippy.log.
- No pushes/PRs/merges or issue filing. KEV-only dependency drafts: docs/kev-upstream-issue-drafts.md. Optional advisory enrichment/filter/raw-slash-route work remains excluded, not advertised.
