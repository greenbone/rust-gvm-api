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
## PR #740 merge-queue Cargo Audit repair

- Merge-group Security run 45183874225 failed because the stack-base lockfile still selected `url` 2.5.8 after RUSTSEC-2026-0592 was published; the patched 2.5.9 update existed only on child PR #743.
- Updated `url` to 2.5.9 and its `form_urlencoded` dependency to 1.2.3, with matching minimal Cargo Vet exemptions. The exact Cargo Audit command, Cargo Deny, Cargo Vet, formatting, diff hygiene, strict workspace/all-target/all-feature Clippy, and the complete all-feature workspace test suite pass. Signed publication and merge-queue rerun remain.

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


## Independent acceptance review

- Re-read all four issues, planned contracts and implementation paths. Found #736 additive metadata inadvertently required GMP 22.8 for existing report detail. Corrected typed unsupported-command handling to omit enrichment only; preserve generic task when snapshot task is missing; all real backend errors still propagate. Regression includes GMP 22.7 report read and no dispatched snapshot/task/config query.
- #736 review validation: focused older-backend and real-backend-error adapter regressions both pass.
- #735 independent review: classic/nonclassic/future/empty/absent typed result, note and override discriminator regression passes; missing result NVT remains absent. Corrected documentation comments that had attached typeMetadata description to type.
- #742 independent review repeated immutable gvmd source/schema and typed-model search: refs/EPSS producers present, no KEV contract found. Corrected producer source anchors. Following explicit parent authorization and fresh authenticated all-state duplicate searches (KEV and “known exploited”), filed gvmd #3254 and rust-gvm #873, and cross-linked API #742 comment 7018718816. No runtime branch publication. Expanded raw refs regression to diverse feed types, escaped URL, CVE compatibility and duplicate/order retention.
- #742 focused typed/REST collection regressions pass; required full-workspace Clippy, formatting and docs validation pass. Full workspace rerun pending final result.
- Independent read-only review of parent b02/732-agent-os-metadata (dcd4706; #730–732) against issue acceptance and specs found no concrete blocker in scheduler compatibility/range mapping, inventory-token authentication/capabilities/status/confidentiality/cache, or OS omission/open-string mapping. Parent worktree and branches were not changed.

# API #741 report result isolation

- 2026-11-19: Rooted at PR740 head9a5664f. Read contract/code and pinned local gvmd producer.
- Identified protected `_and_report_id` control in gvmd source; filed rust-gvm#871 for typed composition and strict comparator validation, rather than parsing GMP in gateway.
- Adding defense-in-depth response scope invariant: backend cross-report rows fail the whole page with sanitized502; UUID case-equivalent identities remain accepted.
- Waiting on upstream helper plus shared immutable dependency adoption before final adapter integration/testing/publication.

- Adopted upstream immutable feature stack b6f33da; reviewed 270-method typed facade (three newly blocked REST adopters). Building isolated target; native compiler provisioned inside workspace only.
- Integrated typed compose_report_results_filter at both report result read paths; saved terms resolve first and no filter_id is forwarded. Added transport regressions for caller/saved boolean alternatives, report-scoped counts across pages, and rejection before result execution for unsupported or endpoint-owned predicates.
- Focused testing initially used CLI path patches; final manifest and regenerated lockfile now pin all five upstream crates to published immutable 64b9d01820c8b4715265bbae06293462c24fdfe7 (rust-gvm#872), without local patches. All 120 focused adapter/scope tests and strict workspace Clippy passed; full workspace tests running.

- Parent full-suite retries exposed two stale dependency-adoption expectations: application fixture intentionally lacks severity (kept absent), and the exact dependency invariant now pins64b9d018. Both corrected without weakening production contracts. The fixture correction is already present in stack730 and need not be cherry-picked into final741. Full-suite retry in progress.
- Full workspace `cargo test --workspace --locked` completed successfully, including OpenAPI parity and doctests; zero failures. Strict workspace Clippy was already successful at this code state. Integration into issue-specific final stack pending sibling slice completion.

## API #741 quote-override transport follow-up
- Independent upstream parser review corrected quote semantics in rust-gvm #872 at fa990ad0c8aab4609dec7999f01e9922713fe71b. Updated all five manifest/lock pins and exact architecture invariant; no local protocol parsing or parent edits.
- Added transport regressions for disguised scope/window controls and inclusive operators in both inline and resolved saved terms; assert InvalidInput and absence of get_results. Real double-quoted literals, terminal backslashes and ordinary apostrophes remain dispatchable using fixture playback.
- Confirmed negative regression fails against 64b9d018 (old helper dispatches and returns an empty successful page). At fa990ad all six report-scope transport tests pass (including OR, counts/paging, saved and inline reserved/quote cases), as do all 22 architecture tests. Exact manifest/lock coherence and fmt/diff checks pass.
- Required `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` passed (only pre-existing Cargo manifest unused-dependency warnings). Tooling sourced /workspace/api-agent-env, isolated CARGO_TARGET_DIR=/workspace/.cache/api-filter-target. No parent checkout or specifications modified; no push/PR requested.
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

## 2026-11-20 — issue #741 review
- Review correctly identified that successful rows missing their report association bypassed the response-side scope check.
- Reject absent and mismatched report identities before conversion for both report-results and embedded-report paths; preserve case-equivalent UUID acceptance.
- Added successful-response fixture coverage without a report association through both typed adapter paths. Updated the published report error contract to describe unverifiable rows.


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

## PR #743 CI repair (2026-11-23)

- Inspected failed Security and E2E runs at head 5d45e33. Causal failures were missing cargo-vet coverage for the jsonschema dev dependency, audit reports being read only through scan-shaped typed commands, and the E2E summary-NVT DTO omitting the specified open type discriminator.
- Added typed audit-report fallback and projection without local GMP construction or backend-message matching. Added an adapter regression proving audit reports use get_audit_report after the scan-shaped miss.
- Added the open NVT type to full/summary E2E DTOs and a regression that preserves the EPSS-free summary boundary.
- Regenerated minimal cargo-vet exemptions. `cargo vet --locked`, formatting, diff hygiene, strict workspace/all-target/all-feature Clippy, both focused regressions, and the complete all-feature workspace test suite pass. Rebase onto the current PR base completed cleanly; signed commit, push, and exact-head CI are pending.
- Exact-head CI run 45171334880 exposed that the new E2E DTO regression was missing from the mandatory evidence inventory. Added it under the existing `rest.dto.summary-nvt-boundary` contract. The full evidence gate now passes with 46 inventoried Rust tests; publishing a signed follow-up commit before monitoring replacement CI.
