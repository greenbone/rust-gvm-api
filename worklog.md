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
## Review correction #730 (2026-11-20)

Reject explicit null maxDelaySeconds on all agent schedule and group inputs; preserve omission, zero and GMP bounds. Add HTTP 400/no-backend-dispatch regressions and REST boundary coverage.

Validation (source /workspace/api-agent-env; CARGO_TARGET_DIR=/workspace/.cache/api-agent-target): cargo test -p gvm-gateway-rest --lib (200 passed, including generated/curated OpenAPI parity); cargo test -p gvm-gateway --test agents (5 passed, including all four invalid-delay HTTP entry points with zero backend commands); cargo fmt --all -- --check; cargo clippy --workspace --all-targets --all-features -- -D warnings. All passed.
## Review correction #731 (2026-11-20)

Publish the inventory-token 200 Cache-Control header with const no-store in generated OpenAPI; assert generated/curated header parity.

Validation (source /workspace/api-agent-env; CARGO_TARGET_DIR=/workspace/.cache/api-agent-target): cargo test -p gvm-gateway-rest --lib (201 passed, including inventory-token generated/curated header parity); cargo test -p gvm-gateway --test agents (5 passed, including runtime inventory-token no-store success/rejection behavior); cargo fmt --all -- --check; cargo clippy --workspace --all-targets --all-features -- -D warnings. All passed. Reused the existing target cache; refreshed local source mtimes after cross-worktree stale dependency artifacts initially hid this slice's token service/error additions.

## PR #747 review correction (2026-11-20)

- Isolated at manifest head 4c5a82f63c84693c2e16ba071471b800014fd176; parent stack untouched.
- Authenticated review confirms missing runtime 409/500/504 outcomes in both AD verification contracts. Added generated and curated problem responses, explicit status parity, six saved/inline HTTP error cases with no-store and sensitive-payload exclusion, plus typed adapter conflict/timeout sanitization coverage.
- Verification in progress using report-test-env and an isolated copy of the existing cache; no dependency changes.

- Final verification: saved/inline HTTP + published-status parity 4 passed (six 409/500/504 HTTP combinations), REST 206 passed, gateway contract 27 passed, two focused typed AD adapter regressions passed; fmt check and required strict workspace/all-target/all-feature Clippy passed.
- Shared-target concurrency initially produced stale cross-head artifacts; isolated copied cache and source freshness resolved it without dependency or parent-stack changes.

## PR #748 review correction (2026-11-20)

- Isolated at manifest head cc9e750e901c84ae416b8793b395287251964b1f; parent stack untouched.
- Authenticated review confirms generated AD search 200 lacks the runtime/curated Cache-Control header. Added a machine-readable string const no-store header, a REST schema assertion, and served-generated/curated header-schema parity.
- Verification in progress using report-test-env and an isolated copy of the existing cache; no dependency changes.

- Final verification: full REST suite 214 passed (including generated no-store schema), gateway contract 30 passed (including served-generated/curated header-schema parity); fmt check and required strict workspace/all-target/all-feature Clippy passed. Used an isolated copy of the existing report cache to avoid other worktree artifacts; no dependency changes.


## PR #743 response delay review (2026-11-20)
- Isolated worktree based on exact a1c3f29611a2e37611e64fba38c4887a5ed95625; no shared stack edits.
- Split signed-range write schedules from full-u32 response observations; keep non-null input validation. Review and owning specification read.
- Scanner responses at this slice do not expose agent defaults or reuse AgentConfigResponse; scanner defaults writes reuse the constrained request model.
- Response schedule schema is now AgentScheduleResponse (0..u32::MAX); structured write object is AgentScheduleInput (0..i32::MAX). Group read/write schemas retain separate existing names. No runtime read validation, casts, or truncation added.
- Sidecar regressions cover typed GMP agent/group projections at 2147483648/u32::MAX, complete REST response serialization validated against generated and bundled curated schemas, exact field range/type boundaries, and HTTP rejection with no dispatch for u32::MAX (including defaults writes). Added dev-only jsonschema validator with remote resolution features disabled rather than extending the primitive-only enum test helper.
- Verified using /workspace/api-agent-env: cargo test -p gvm-gateway-rest (202 passed, including generated/curated OpenAPI parity); cargo test -p gvm-gateway-gvmd typed_agent_and_group_delay_observations (1 passed); cargo test -p gvm-gateway --test agents --test contract (5 + 27 passed); cargo fmt --all -- --check; cargo clippy --workspace --all-targets --all-features -- -D warnings (exit 0). Existing Cargo manifest unused-dependency notices remain, with no Rust/Clippy diagnostics. Final logs: /workspace/reviews/api-727-742/review21-743-{rest,projection,http,fmt,clippy}-final.log.
- No push, PR mutation, or review comment; scoped commit is for parent merge-forward/restack and CI.
## Review correction #876 / API #744 (2026-11-20)
- Published owning parser fix at greenbone/rust-gvm#877, bcac05492244f55df51b971184cc63ddc7bd7bed, incremental on #872/fa990ad so the protected report-result filter helper is retained. No PR merged.
- Adopted all five exact dependency pins, lock source identities, architecture revision guard and facade snapshot revision together.
- Added typed adapter and real HTTP regressions for self-closing/explicit empty and whitespace-only successful backend tokens, including XML whitespace/entity references; assert sanitized backend-unavailable/502, no token response, and HTTP no-store.
- No gateway parser, normalization or trim workaround. Running focused tests, architecture guard, full-feature strict workspace Clippy and fmt in dedicated /workspace/.cache/api-876-target.
- Verification passed: adapter inventory-token tests 4/4; actual HTTP inventory-token tests 2/2 (six malformed payload shapes plus valid issuance/auth/body checks); REST cache/OpenAPI tests 2/2; architecture guards 22/22; cargo clippy --workspace --all-targets --all-features -- -D warnings; cargo fmt --all -- --check; git diff --check. Existing manifest-only unused-dependency warnings remain.
- The reviewed client facade source is unchanged between old pin and bcac054, so the existing 270-method inventory/dispositions remain valid; updated snapshot, ledger and adoption-note revision metadata atomically. Specifications left unchanged.
- API branch fix/744-adopt-876 is based exactly on a8c84cb0bd22d0ab48190a92fb984a80dfef24cc; local commit only, no API push/comment/merge. Parent must retain bcac054 when restacking #744/#754 rather than reverting to #754's older fa990ad pin.
## 2026-11-20 — Credentials route-inventory review
- Removed the not-yet-implemented object-search row from the #727/PR747 Credentials inventory and added both implemented saved/inline verification rows.
- Added a documentation gate comparing that inventory with the exact slice's curated route/method contract, plus three regression tests covering the reported drift and the later search introduction.
- Verified documentation at124paths/202operations, all three documentation regressions, Rust formatting and diff checks.
## 2026-11-20 — Search inventory boundary
- Added the object-search inventory row in its owning #728/PR748 slice, after verification routes are introduced in747.
- The new slice-aware documentation gate passes at125paths/203operations.
