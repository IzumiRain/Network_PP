---
id = "ref/waf/audit-log-parts"
kind = "configuration"
title = "Audit logging and log parts"
category = "reference/security/waf/logging"
tags = ["waf", "modsecurity", "logging", "audit-log"]
version = "ModSecurity 2.9 / v3"
---
ModSecurity audit logs record full transactions; log parts select which sections are written.

## Example

```apache
SecAuditEngine RelevantOnly
SecAuditLogType Serial
SecAuditLog /var/log/modsec_audit.log
SecAuditLogParts ABFHZ
```

## Notes

- `SecAuditEngine` is Off / On / RelevantOnly (only transactions that matched a rule - the usual production choice).
- Parts: A audit header, B request headers, C request body, E response body, F response headers, H trailer (rule messages), Z end boundary. Default is `ABCFHZ`.
- `SecAuditLogType Serial` writes one file; `Concurrent` writes per-transaction files - required for high volume and for tools that index single events.
- Part H is what tuning work reads: matched rule IDs, messages and anomaly score summary.

Source: ModSecurity Reference Manual (OWASP) — https://owasp.org
