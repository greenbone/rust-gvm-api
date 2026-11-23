# Technology Preview API migration notes

## Public API base path

The versioned API moves from `/api/v1` to `/gvm/public/v1`. Update client
URLs and reverse-proxy routing accordingly; the old prefix is not an alias or
redirect. OpenAPI and browser documentation now live at
`/gvm/public/v1/openapi.json` and `/gvm/public/v1/docs`. Resource `Location`
headers and export-job result links use the new prefix.

The application-owned liveness and readiness endpoints move to `/gvm/health`
and `/gvm/ready`. Update deployment probe URLs from `/health` and `/ready`;
the former root paths are not retained as aliases. Both probes remain
unauthenticated and unversioned.

Ingress routing should forward `/gvm/...` to the API service without stripping
`/gvm`: the service routes the full path, including `/gvm/public/v1/...`.

## Authentication modes

Protected resource operations accept either a persistent bearer session or
request-scoped HTTP Basic credentials. Create a persistent session with
`POST /gvm/public/v1/session`; inspect or delete that session with bearer-authenticated
`GET` or `DELETE /gvm/public/v1/session`. When Basic credentials are sent directly to
a protected resource operation, the gateway creates a backend context for that
request and tears it down before returning.

## Removed ticket surface

The former `GET /gvm/public/v1/tickets` and `GET /gvm/public/v1/tickets/{id}` Technology
Preview routes have been removed. They now behave like other unknown routes and
return the standard `404` problem response. Integrations should manage tickets
through their external ticketing system rather than through this gateway.

## Report import

`POST /gvm/public/v1/reports` imports one XML report into an existing import task. It
requires `taskId`, accepts optional `inAssets`, enforces a 10 MiB body limit,
and returns `201 Created` with the canonical report `Location`. The XML body is
opaque to the gateway; typed rust-gvm code validates and constructs the GMP
request.

## Explicit scan-configuration and policy bases

The Technology Preview create contract now matches gvmd's copy-based
configuration lifecycle. `POST /gvm/public/v1/scan-configs` requires
`baseScanConfigId`, identifying an active scan configuration, and
`POST /gvm/public/v1/policies` requires `basePolicyId`, identifying an active policy.

The gateway no longer accepts name-only creation and never chooses a base
automatically. A missing or malformed selector, a nonexistent or trashed base,
or a base from the other configuration family returns the standard `400`
RFC 9457 invalid-input response.

Scan-configuration request example:

```json
{
  "name": "Custom full and fast",
  "comment": "Local copy",
  "baseScanConfigId": "daba56c8-73ec-11df-a475-002264764cea"
}
```

Policy request example:

```json
{
  "name": "Custom compliance policy",
  "comment": "Local copy",
  "basePolicyId": "085569ce-73ed-11df-83c3-002264764cea"
}
```

## Task host ordering

`hostsOrdering` is no longer accepted by task or audit create/modify requests.
Pinned gvmd versions do not parse a request-side host-ordering field, so the
previous Technology Preview input was misleading. Response-side
`hostsOrdering` remains unchanged when gvmd reports it.
