# Report projection adoption

## #734 applications

GET /reports/{id}/applications preserves optional hostsCount, occurrences and open-string threat from typed upstream application summaries. Zero is distinct from absence; severity remains server-calculated. No live Agent qualification is claimed.

## #735 NVT type

JSON type is an optional open string. Ordinary results (including report embedded results) map typed NvtRef.type_; notes/overrides map typed NvtReference.type_, correcting the former misprojection as family. Aggregate vulnerability and closed-CVE sources lack a typed discriminator and keep it absent. Metadata is unchanged; missing NVTs remain missing.
