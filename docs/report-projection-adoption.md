# Report projection adoption

## #734 applications

GET /reports/{id}/applications preserves optional hostsCount, occurrences and open-string threat from typed upstream application summaries. Zero is distinct from absence; severity remains server-calculated. No live Agent qualification is claimed.

## #735 NVT type

JSON type is an optional open string. Ordinary results (including report embedded results) map typed NvtRef.type_; notes/overrides map typed NvtReference.type_, retaining the legacy family alias for backward compatibility. Aggregate vulnerability and closed-CVE sources lack a typed discriminator and keep it absent. Metadata is unchanged; missing NVTs remain missing.

## #736 report snapshot scan configuration

Detail GET /reports/{id} and gateway JSON exports preserve optional task.scanConfig (UUID/name) using typed GetScanReportRequest / StructuredReportTask.scan_config. Generic GET /reports list task references omit scanConfig: upstream generic Report has only NamedEntity and would need an upstream extension for list coverage. No current task lookup, XML parsing or audit/delta surface is added. Existing metadata and explicit result pagination are retained. This structured helper requires upstream GMP 22.8 support; typed unsupported-command capability results omit this optional enrichment and retain the generic task reference. An absent snapshot task also retains that reference without a configuration. Permission, parse and backend failures propagate, never fabricated metadata.

