# KEV upstream dependency drafts for #742

Not filed by this worker. Parent owns filing and cross-linking. Parent reports authenticated all-state KEV searches found no existing gvmd/rust-gvm issue; do not duplicate API #682/#685 or rust-gvm #825 (generic references/aggregates).

## Producer issue — greenbone/gvmd

**Title:** Expose feed-backed per-CVE CISA KEV metadata through GMP results and NVTs

### Problem / evidence

API issue https://github.com/greenbone/rust-gvm-api/issues/742 requests per-CVE KEV observations alongside EPSS, usable without internet access from API consumers. Existing generic references do not establish catalogue membership, date added, remediation due date or ransomware campaign usage.

Static inspection of gvmd revision `150a4cfad39d00f45ef147c35e1a5f27f415c8e9` found no `kev`, `known.exploit`, `known_ransomware` or `cisa` producer/feed contract in `src/`. Its [result reference/EPSS producer](https://github.com/greenbone/gvmd/blob/150a4cfad39d00f45ef147c35e1a5f27f415c8e9/src/gmp.c#L9480) emits CERT/generic refs and EPSS; neither is KEV evidence. This is source review, not a claim about unpublished feed products or live builds. Please identify an existing supported contract if one exists.

### Requested upstream contract

- Ingest/distribute actual KEV catalogue data on the scanner/feed side with air-gapped/feed-update semantics; do not make downstream gateways download CISA data.
- Expose typed-documentable per-referenced-CVE observations in ordinary result NVT and catalog NVT GMP projections: CVE ID, date added, due date and open-string known ransomware campaign use where available.
- Define no feed loaded/stale data/not-listed states and omission semantics; absence must not fabricate a negative membership result. Document feed provenance/update timestamp and authorization/capability behavior.
- Define schema and deterministic populated/absent/not-listed/multiple-CVE/unknown-string tests, including CVEs whose highest EPSS and KEV membership differ.
- Preserve all existing severity, EPSS and raw references; no implicit result filtering or aggregate endpoint scope expansion.

Blocks only the KEV portion of API #742; ordinary raw refs can ship independently. Coordinate typed adoption in rust-gvm once the producer contract is defined.

Agent: b02-agent-035

## Typed library issue — greenbone/rust-gvm

**Title:** Add typed per-CVE KEV response support once gvmd/feed producer contract exists

### Evidence / blocked consumer

API https://github.com/greenbone/rust-gvm-api/issues/742 needs actual upstream KEV observations, not a web lookup or an inference from CVEs, tags or EPSS. At immutable revision `b6f33da1224aa4950f39dc3db0e6d5607e0e0f89`, [result NvtRef](https://github.com/greenbone/rust-gvm/blob/b6f33da1224aa4950f39dc3db0e6d5607e0e0f89/crates/gvm-gmp/src/responses/result.rs#L34), [catalog Nvt](https://github.com/greenbone/rust-gvm/blob/b6f33da1224aa4950f39dc3db0e6d5607e0e0f89/crates/gvm-gmp/src/responses/nvt.rs#L18) and common models preserve generic reference entries and EPSS but expose no KEV model or typed feed source. Static source searches found no KEV decoder. Link the producer issue above when filed; this is blocked on its agreed wire/feed semantics, not permission to invent XML.

### Acceptance

- Once producer contract is available, add optional typed KEV observations keyed by CVE, retaining date-added/due-date and unknown ransomware usage strings without defaulting missing observations.
- Preserve older responses, missing feed, non-listed CVEs and multiple-CVE relationships; document stale/unknown availability according to producer semantics.
- Add parser/model tests and stateful mock fixtures using explicit loaded feed observations; do not synthesize membership from tags/refs or fetch CISA over the network.
- Expose typed client/facade support and capability/version information if required by the producer, enabling REST ordinary result/catalog NVT adoption without gateway XML parsing.
- Keep #825 aggregate references separate; no duplicate generic refs issue or unsolicited aggregate KEV scope.

No live KEV qualification is claimed. Blocks only KEV in API #742.

Agent: b02-agent-035
