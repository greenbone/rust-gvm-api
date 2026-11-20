# Planned REST contracts: GMP feature adoption (#727–736)

> **Design supplement only — not implemented REST behavior.** This document
> specifies downstream adoption work for rust-gvm #847–857. It does not add routes,
> parameters or response fields to the shipped API, complete API issues #727–736,
> or change the dependency pin. Clients must use the implemented
> [OpenAPI root](openapi.yaml) and [REST design](openspec.md) for current behavior.
> In particular, `anonymized=true` is not currently an API anonymization guarantee.

## 1. Status, evidence and ownership

The implementation stack in **rust-gvm** supplies typed requests, response
models, capability checks and deterministic/mock tests. That is not downstream
REST integration or live gvmd/AD qualification. This supplement was checked on
2026-11-19 against rust-gvm stack revision
[`b6f33da1224aa4950f39dc3db0e6d5607e0e0f89`](https://github.com/greenbone/rust-gvm/commit/b6f33da1224aa4950f39dc3db0e6d5607e0e0f89).
The source/schema review pins gvmd
[`150a4cfad39d00f45ef147c35e1a5f27f415c8e9`](https://github.com/greenbone/gvmd/commit/150a4cfad39d00f45ef147c35e1a5f27f415c8e9).
Neither pin proves a feature exists in every released/build configuration.

| Planned gap | API issue | Typed rust-gvm prerequisite |
| --- | --- | --- |
| AD credentials and verification | [#727](https://github.com/greenbone/rust-gvm-api/issues/727) | [#847](https://github.com/greenbone/rust-gvm/issues/847) |
| AD object discovery | [#728](https://github.com/greenbone/rust-gvm-api/issues/728) | [#848](https://github.com/greenbone/rust-gvm/issues/848) |
| Target provenance | [#729](https://github.com/greenbone/rust-gvm-api/issues/729) | [#849](https://github.com/greenbone/rust-gvm/issues/849) |
| Structured agent schedules/delay | [#730](https://github.com/greenbone/rust-gvm-api/issues/730) | [#851](https://github.com/greenbone/rust-gvm/issues/851) |
| Agent inventory token | [#731](https://github.com/greenbone/rust-gvm-api/issues/731) | [#852](https://github.com/greenbone/rust-gvm/issues/852) |
| Agent OS metadata | [#732](https://github.com/greenbone/rust-gvm-api/issues/732) | [#853](https://github.com/greenbone/rust-gvm/issues/853) |
| Support-bundle anonymization | [#733](https://github.com/greenbone/rust-gvm-api/issues/733) | [#854](https://github.com/greenbone/rust-gvm/issues/854) |
| Application counts/threat | [#734](https://github.com/greenbone/rust-gvm-api/issues/734) | [#855](https://github.com/greenbone/rust-gvm/issues/855) |
| Result-NVT discriminator | [#735](https://github.com/greenbone/rust-gvm-api/issues/735) | [#856](https://github.com/greenbone/rust-gvm/issues/856) |
| Report scan-config reference | [#736](https://github.com/greenbone/rust-gvm-api/issues/736) | [#857](https://github.com/greenbone/rust-gvm/issues/857) |

rust-gvm [#850](https://github.com/greenbone/rust-gvm/issues/850) corrects sensor
port validation, not a REST gap: scanners remain **read-only**. Sensor port zero
must not become justification for scanner create/update/verify routes. Preserve
all other [surface dispositions](../../docs/upstream-surface-dispositions.md),
including ticket removal, deferred agent integration configuration, and omitted
specialized audit/delta exports and administration operations. The accepted
release's machine-readable ledger remains unchanged until dependency adoption.

### Contract notation

All paths below are **proposals** relative to `/api/v1`, not route registration.
JSON names are explicit REST choices, not automatic serialization of upstream
Rust types. Optional response values are omitted, not replaced with null/empty/
zero defaults. A typed `None` stays absent; `Some(0)`, `Some(false)` and
`Some([])` remain real observations. Empty strings normalized to `None` by the
upstream parser cannot be reconstructed downstream. Requests preserve omitted
options rather than eagerly writing server defaults. Unless a section permits
it, JSON null is invalid, not an implicit clear operation.

Use existing session serialization/backpressure and Basic-or-Bearer policies
for protected operations; no new anonymous endpoints. Proposed sensitive token
responses additionally require no-store handling. No feature may construct GMP
XML, parse XML, locally bind LDAP, issue/sign inventory tokens, infer DN
containment or emulate a missing backend command in this repository.

## 2. Common prerequisites and failures

Before implementing each issue:

1. Adopt an immutable, reviewed rust-gvm revision/release containing its typed
   support across all workspace components; refresh the facade inventory and
   disposition ledger deliberately, not as part of this docs-only change.
2. Add domain ports/models, application use cases, typed gvmd conversions,
   REST validation/serialization and focused tests. Request validation precedes
   backend dispatch. Reuse upstream command identity and capability discovery.
3. Publish curated split OpenAPI and generated handler schemas together **only
   with working runtime integration**. Keep the implemented route inventory and
   user documentation in sync. A draft must never be merged into the served
   OpenAPI as if implemented.
4. Qualify against the relevant real gvmd build and (for AD) a controlled LDAP/TLS
   environment before claiming live interoperability. Deterministic fixtures
   do not prove destination policy, TLS or authorization.

Capability discovery is internal, not a new public help/features route. AD
verification/search require positive XML-help discovery; do not invent a GMP
version minimum from commit dates. AD create/copy/modify additionally require
gvmd's explicit `verify_active_directory_credential` permission, including for
administrators. Generic credential operations keep their existing capabilities;
AD discovery is not a blanket gate on unrelated credential types.
Inventory-token retrieval requires GMP 22.8 **and** positive help discovery,
plus the existing `get_agents` permission. Existing agent operations retain
upstream GMP 22.8 gates. Added optional read projections need no guessed
version gate; older server payloads simply omit them. Backend support for new
write selectors must be qualified; never report success after dropping one.

The following are planned HTTP mappings using RFC 9457 `application/problem+json`
with the existing problem fields (`type`, `code`, `title`, `status`, `detail`,
optional `instance`). They do not change the current global error contract:

| Condition | Planned handling |
| --- | --- |
| Invalid UUID, type/range/combination, malformed boolean | `400 bad_request`, no command dispatch |
| Missing/invalid gateway authentication | `401` |
| Backend permission denial (even GMP status 400 with permission-denied semantics) | `403`, not misclassified as caller syntax |
| Saved credential/resource not found or hidden | Existing backend-authorized `404`; do not leak hidden resources |
| Positive capability absent/unsupported build or command | `501` capability failure; never emulate |
| Known command temporarily cannot issue an inventory token | `503`; distinguish from unsupported command |
| Resource/state conflict | Existing `409` mapping |
| Connection/transport or malformed successful upstream payload | `502`; never invent a success body |
| Backend timeout | `504` |
| Backend internal token-generation/operation failure | `500`, sanitized detail |

Preserve typed error status/category for mapping; do not copy arbitrary upstream
status text, DNs, hosts, secrets or LDAP diagnostics into public problem details.
AD source errors must be classified with fixtures rather than assuming every GMP
400 is bad syntax. Mapping unrecognized LDAP/TLS/destination failures to a
stable safe problem code is an implementation decision to finalize under #727/
#728; such errors must never become successful verification or an empty search.

Bind passwords and inventory tokens are confidential in request/response capture,
Debug, logs, spans, metrics labels, audits and error bodies. AD DNs, directory
objects, connection metadata, certificate content and support bundles are
sensitive too: return only authorized data and exclude payloads from telemetry.
Use safe operation/resource identifiers for diagnostics. Backend wire-trace
redaction does not make gateway JSON logging safe.

## 3. AD credentials and verification — #727

Extend existing credential create/update/read shapes with type `ad` and a nested
`activeDirectory` object. `CredentialKind::ActiveDirectory`/typed credential
requests supply the source; never use an `Unknown` type workaround.

| JSON field in `activeDirectory` | Type and semantics |
| --- | --- |
| `host` | Nonempty string; required on create/inline verify |
| `port` | Optional integer 1–65535; omission delegates default to gvmd (636 for `ldaps`, 389 for `starttls`) |
| `clearPort` | Modify-only boolean, default false; true restores server mode default, mutually exclusive with `port` |
| `bindDn`, `baseDn` | Nonempty strings; required on create/inline verify |
| `bindPassword` | Nonempty write-only string; required on create/inline verify; never a response field |
| `tlsMode` | Required `ldaps` or `starttls` on create/inline verify; no plaintext LDAP mode |
| `caCertificate` | Optional PEM string; explicit empty string on modify clears stored CA |

Modify is partial: omitted fields preserve stored values; no empty host, bind DN,
base DN or password. `clearPort=true` is invalid on create/inline verify.
Require explicit `ad` type on create when supplying AD fields. Partial modify
targets the saved credential ID and does not require a redundant type field,
matching the upstream partial-modify API. Read metadata includes optional
`host`, `port`, `bindDn`, `baseDn`, `tlsMode`, `caCertificate`, but not `bindPassword` or
`clearPort`. Read `tlsMode` is an open string so future backend values survive.

Proposed verification resources/actions:

- `POST /credentials/{id}/actions/verify-active-directory`, no inline fields:
  `VerifyActiveDirectoryCredentialRequest::Saved`.
- `POST /active-directory/credential-verifications`, complete
  `activeDirectory` body, no credential ID: `Inline`, no persistence.
- Both return `204 No Content` on status-only success; no fabricated connection
  details, credential resource or Location. The saved action is a deliberate
  controller exception to the resource-first rule; the inline operation creates
  no durable saved credential. Final route naming is subject to #727 review.

Tests must prove required/type/range validation, escaped values, partial updates,
CA clearing, port clearing/default omission, saved-vs-inline mutual exclusion,
status-only success, denied/disabled/destination/TLS failures, and password
redaction through capture of gateway requests, responses and diagnostics. No
built-in mock success should be represented as a real LDAP bind.

## 4. AD object discovery — #728

Proposed `POST /active-directory/object-searches` accepts a bounded JSON search
body; this avoids sensitive DN selectors in access-log query strings. It is a
read-only controller search (no stored search/target/task), returning `200` and
no Location. This is an explicit proposed action-style exception, not a reason
to expose arbitrary GMP commands. Requires saved `credentialId` (UUID).

| Optional JSON input | Contract/default when omitted |
| --- | --- |
| `scope` | `base`, `onelevel`, `subtree`; backend default `onelevel` |
| `maxResults` | Integer 1–1000; backend default retained, not guessed |
| `includeDisabledComputers` | Boolean; backend default true |
| `targets`, `tasks`, `details` | Booleans; backend defaults false; `tasks=true` requires explicit `targets=true` |
| `baseDn` | Nonempty string; default credential base DN |
| `groupDn` | Nonempty string; selects direct members, not recursive group expansion |
| `objectTypes` | Nonempty array of `ou`, `container`, `computer`, `group`; omitted defaults to OU/computer/group (not container); duplicate selections have set semantics |

Map into `GetActiveDirectoryObjectsRequest`. gvmd owns DN containment, case
handling, destination and visibility policy. No DN normalization or local
comparison. No implicit persistence, client-side completeness claim or invented
page cursor; this is a limited fetched subset, not REST page/perPage pagination.

Proposed response: `{ "objects": { "truncated": false, "counts": { "ou": 0,
"container": 0, "group": 0, "computer": { "total": 0 } }, "items": [] } }`.
The `objects` envelope itself is optional because the typed response permits
absence: preserve absence rather than inventing an empty successful collection.
Counts are u32 integers for the **fetched subset**, not global directory totals;
optional `computer.disabled` remains absent when excluded/unavailable.

Each item preserves `objectType` (the four request kinds), `name`,
`distinguishedName`, `parentDn`, `description`; the latter two may be empty.
Optional computer metadata: `dnsHostname`, `operatingSystem`,
`operatingSystemVersion`, `servicePrincipalNames` (string array), `disabled`.
Optional `targets` array contains `{id,name,tasks?}`; each optional task array
contains `{id,name,config?:{id,name}}`. Preserve omitted versus explicit empty
SPN/target/task arrays and unavailable disabled flags. Return only references
that gvmd includes as readable/non-trash; do not look up hidden configurations.

Tests: every selector/default and invalid combination, all four object kinds,
escaped DNs, no collection vs empty vs truncated-empty collection, nested fetched
counts, excluded disabled count, populated/details/empty expansions and unreadable
config omission; denial/unavailability/policy failures must not become empty
results. Prove no target/task creation occurs.

## 5. Target provenance — #729

Add optional `source` to existing target create/modify and list/detail
representations, mapping typed request/response `source`. Write values are
`manual` or `active_directory`, matching upstream validation; empty/unknown
values are rejected with 400 before dispatch. Omitted create retains gvmd's
`manual` default; omitted modify leaves stored source unchanged. Reads preserve
an open source string (including future backend values), not a forced `ad`
marker. Older/missing (or parser-normalized empty) source is absent on reads.
Existing GMP `filter` forwarding may select source; do not invent a new REST
filter grammar or locally filter paginated results. Target clone semantics remain
server-owned; no synthetic source assignment.

Tests: `active_directory` provenance survives create/read/update, omitted update
retains it, explicit `manual`, invalid empty/unknown writes, older payload
omission, future source on reads, source filter forwarding and no synthesized
provenance.

## 6. Structured schedules and scheduler delay — #730 (implemented)

The current REST `config.agentScriptExecutor.schedulerCronTime` is an array of
strings. Planned entries are `{ "cron": "0 */6 * * *", "maxDelaySeconds": 30 }`,
mapping `AgentSchedule { cron, max_delay_seconds }` for reads and agent/default
updates. `cron` is a required string; cron interpretation remains backend-owned.
`maxDelaySeconds` is an optional, non-null integer: writes accept 0–2147483647,
while read observations preserve the full typed 0–4294967295 range. Omission
delegates the request default (zero) to gvmd but remains absent on reads; explicit
zero stays zero.
Agent-group `schedulerCronTime` remains a **single string**, not this array;
add optional top-level `maxDelaySeconds` to create/modify/read with the same
read/write ranges and omission rules. Do not conflate group synchronization and
agent executor schedules.

Implemented Tech Preview migration: accept legacy string entries on writes as `{cron: value}`
with omitted delay (as supported by upstream deserialization), while publishing
structured response entries. This response-type change is **not additive** for
existing clients. #730 deliberately adopts an explicit Tech Preview breaking v1 response change, documented in docs/user/migration.md. No negotiation or version endpoint is invented. An empty
schedule array currently emits no scheduler subtree upstream; do not promise
that it clears schedules without separate typed backend support.

Tests: legacy input and structured input, nested multi-entry output, absent vs
zero/nonzero delay, maximum boundary and overflow/negative/noninteger rejection,
group create/modify/read delay, and a compatibility test proving the selected
migration preserves existing client behavior. Verify canonical nested cron
encoding via typed integration, not gateway XML assertions/parsing.

## 7. Agent inventory token — #731 (implemented)

Implemented `POST /agents/inventory-token`, no body or agent ID, returns `200` JSON
`{ "token": "<opaque confidential value>" }` with `Cache-Control: no-store`.
POST reflects confidential issuance, not a cached agent read; this is an explicit
proposed controller exception. No expiration, JWT claims, rotation or signer is
invented; no local issuance, caching, or persistence. Map the no-argument
`GetAgentInventoryTokenRequest` and unwrap `AgentInventoryToken` only for the
intended response. A session token and inventory token are distinct credentials.
This controller exception is implemented in runtime and OpenAPI.

Tests: authorized success and exact no-argument typed dispatch, `get_agents`
denial (source GMP 400 => HTTP 403), absent capability/disabled build,
temporarily unavailable issuance (source 503 => HTTP 503), generation failure
(source 500), missing payload (not a fabricated token), no-store headers and
redaction of tokens in all logs/traces/errors. Deterministic mock tokens are
nonfunctional fixtures, not evidence of working inventory authentication.

## 8. Agent operating-system metadata — #732 (implemented)

Add read-only optional open strings `operatingSystem`, `architecture`, `osCpe`,
`osStatus` on existing Agent list/detail responses. Map the four typed Agent
fields exactly; do not infer platform, validate a closed status enum or invent
an OS from another resource. Upstream absent **and empty** elements decode to
None and should be omitted, not emitted as empty strings. No create/update field
or OS administration endpoint is added.

Tests: all four populated, unknown status/architecture preserved, independent
absence, empty-to-absent normalization and list/detail serialization/schema
coverage. This is a projection change, not a live agent compatibility claim.

## 9. Support-bundle anonymization — #733

Add optional boolean query `anonymized` on the existing
`GET /agents/{id}/support-bundle`; retain `days` and `encryption`. Map to typed
`GetAgentSupportBundleRequest::anonymized` without changing the binary download
or content headers. Omitted, false and true must remain distinct through every
layer. gvmd's documented defaults are encryption=true and anonymized=false;
do not eagerly send defaults, equate encryption with anonymization, or promise
all bundle contents are safe to share. Malformed booleans must return 400 before
backend dispatch. Never silently ignore an explicitly requested privacy option
when backend support is unavailable. Backend anonymization, not a local rewrite
of archives, supplies the guarantee.

Tests: all nine omitted/false/true encryption × anonymized combinations, days
unchanged, malformed input rejection, typed command receives explicit true,
unsupported/error propagation and confidential download/log handling. Current
clients must not rely on this query until #733 is implemented and qualified.

## 10. Report application counts — #734

On existing `GET /reports/{id}/applications` rows add optional `hostsCount` and
`occurrences` (u32 integers 0–4294967295) and `threat` (open string), mapping
`ReportApplicationSummary`. Preserve zero vs absence; upstream empty values
normalize to absent. Retain existing id/name/severity and pagination. Severity
is calculated by gvmd, including agent-derived highest-severity/fallback rules;
never recalculate counts or severity in the proxy.

Tests: nonzero agent-derived aggregates, explicit zero, absent/empty values,
unknown threat and unchanged severity/pagination. Invalid upstream count decoding
must be an upstream failure, not a default zero. Schema/serialization cover all
three optional additions.

## 11. Result-NVT discriminator — #735

Choose JSON **`type`** (not Rust's `type_`) as an optional open string alongside
existing `typeMetadata`. Map `gvm_gmp::responses::NvtRef::type_` without enum
narrowing, default `nvt`, normalization or interpretation of `typeMetadata`.
Add to full and summary NVT projections wherever the supported typed source
provides it: result list/detail, report embedded results, report vulnerabilities
and closed-CVE projections, and note/override NVT summaries. Inventory each
conversion at implementation time: a source providing only a name/reference
must not be supplemented by guessing or querying a different NVT. In the
inspected stack, `ReportVulnerability` and `ReportClosedCve` lack this field;
their synthesized summary references must keep it absent unless upstream adds
typed support. Do not count schema availability as actual source coverage. Catalog NVT
`discovery` and existing summary EPSS omissions remain unchanged.

Tests: classic `nvt`, non-classic, unknown future strings, absent/empty normalized
type, unchanged source-shaped `typeMetadata`, full/summary schema consistency
and every supported nested conversion. Do not fabricate a missing NVT object.

## 12. Report task scan-config metadata — #736

Planned backward-compatible projection: report-specific optional
**`task.scanConfig: {id, name}`**, keeping existing `task.id`/`task.name` and all
unrelated generic ResourceRef shapes unchanged. Both UUID and name must come
from the report payload's typed `StructuredReportTask::scan_config` reference,
not today's task config or report-export `reportConfigId`. Omit when the task
or scan config is absent (older responses, CVE/container/agent tasks); never
synthesize a base config.

Important prerequisite: current gateway list/detail uses generic
`gvm_gmp::responses::Report` with a `NamedEntity` task; the new field lives in
**structured scan/audit report models**, not that generic reference. #736 must
choose typed structured-report consumption for existing detail/JSON export,
or obtain an upstream typed generic-report extension if list responses are to
carry it. Do not parse raw XML here, N+1-fetch current task configuration, or
claim the generic list already preserves it. Raw XML export pass-through is
not structured REST adoption.

Initial proposed scope: `/reports/{id}` detail and API JSON exports produced
from that report model. `/reports` list coverage is explicitly pending the typed
source decision; document the final endpoint matrix before release, preserving
all other report pagination/results behavior. No new audit/delta endpoint or
export selector is authorized by this metadata change.

Tests: populated UUID/name and escaped names, absent task/config, config-less
and old reports, unchanged unrelated task references, detail plus JSON-export
serialization and schema; prove values reflect the report snapshot even when
current task configuration differs. If list coverage is added, test typed
availability and absence there explicitly.

## 13. Acceptance and remaining decisions

The tests above are **required future acceptance criteria**, not tests added or
passed by this documentation change. For every implemented issue, exercise
REST request -> domain/application -> typed adapter and typed response ->
REST JSON, malformed input/no dispatch, capability/permission/error cases,
confidentiality and curated/generated OpenAPI parity. Update the
[test specification](test-spec.md) and user reference only with implemented
behavior. Keep checks asserting scanner writes and ticket routes absent.

Decisions remaining before implementation/release:

- #727/#728/#731: finalize proposed route names, method/controller exceptions,
  exact safe problem codes for LDAP/TLS/policy errors, request size limits and
  sensitive response capture/cache policies. No speculative numeric limits here.
- #730: select the versioned/negotiated response migration; accepting legacy
  input alone does not solve the response-breaking change.
- #736: finalize typed read strategy and list/detail/JSON-export coverage matrix;
  report-specific projection is proposed, not an implemented generic ref change.
- All issues: select released immutable dependency baseline and record real
  backend/build qualification. Existing enterprise agent E2E tracking
  [#680](https://github.com/greenbone/rust-gvm-api/issues/680) remains relevant;
  this document supplies no live AD or Agent Control qualification.
