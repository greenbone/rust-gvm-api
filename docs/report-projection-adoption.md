# Report projection adoption

## #734 applications

GET /reports/{id}/applications preserves optional hostsCount, occurrences and open-string threat from typed upstream application summaries. Zero is distinct from absence; severity remains server-calculated. No live Agent qualification is claimed.

## #735 NVT type

JSON type is an optional open string. Ordinary results (including report embedded results) map typed NvtRef.type_; notes/overrides map typed NvtReference.type_, retaining the legacy family alias for backward compatibility. Aggregate vulnerability and closed-CVE sources lack a typed discriminator and keep it absent. Metadata is unchanged; missing NVTs remain missing.

## #736 report snapshot scan configuration

Detail GET /reports/{id} and gateway JSON exports preserve optional task.scanConfig (UUID/name) using typed GetScanReportRequest / StructuredReportTask.scan_config. Generic GET /reports list task references omit scanConfig: upstream generic Report has only NamedEntity and would need an upstream extension for list coverage. No current task lookup, XML parsing or audit/delta surface is added. Existing metadata and explicit result pagination are retained. This structured helper requires upstream GMP 22.8 support; typed unsupported-command capability results omit this optional enrichment and retain the generic task reference. An absent snapshot task also retains that reference without a configuration. Permission, parse and backend failures propagate, never fabricated metadata.

## #742 generic references and KEV dependency

Ordinary results, embedded report results and catalog NVTs preserve generic refs entries with raw type/id, including unknown future types. Empty upstream collections are omitted. Existing cves extraction remains unchanged. Aggregate reference coverage remains tracked by #685 / upstream #825; it is not fabricated from ordinary records.

KEV alone is blocked: rust-gvm b6f33da has typed generic references and EPSS but no typed KEV model; inspected gvmd producer/schema provides no KEV feed ingestion or per-CVE KEV metadata. No KEV field is advertised and no external CISA download or XML workaround is used. Exact producer/typed dependency drafts accompany this change. Existing #682/#685/#825 are reference work, not substitute KEV tracking. Optional advisory enrichment/filtering and raw multi-segment legacy advisory path handling are not implemented by this slice; existing route handling is unchanged.


## Endpoint/source matrix

| Surface | NVT type | Generic refs | Report task scanConfig |
| --- | --- | --- | --- |
| /results list/detail | Typed result NVT | Typed result refs | Not applicable |
| /reports/{id}/results | Typed result NVT | Typed result refs | Not applicable |
| /reports/{id} embedded results | Typed result NVT | Typed result refs | Structured report task snapshot |
| Gateway JSON report export | Typed result NVT | Typed result refs | Same typed detail snapshot |
| /reports list | No embedded results | No embedded results | Absent: generic source has no typed field |
| /reports/{id}/vulnerabilities, closed-cves | Absent: source unavailable | Absent: #685/#825 | Not applicable |
| Notes/overrides NVT summaries | Typed common reference | Absent: source unavailable | Not applicable |
| /nvts list/detail | Unchanged catalog model | Typed catalog refs | Not applicable |

No KEV JSON field is published without real feed-backed typed data.
