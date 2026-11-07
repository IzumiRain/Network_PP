---
id = "std/security-plus/owasp-a01-broken-access-control"
kind = "protocol"
title = "OWASP A01:2021 — Broken Access Control"
category = "reference/security/security-plus/owasp"
tags = ["owasp", "a01", "access-control", "idor"]
version = "OWASP Top 10:2021"
---
Broken access control means users can act outside their intended permissions because authorization is missing, client-side only, or inconsistently enforced. It ranks first in the 2021 list.

## Typical example

```text
GET /api/invoices/1321  ->  user A changes the ID to 1322 and reads user B's invoice (IDOR)
```

## Defenses

- Deny by default except public routes; enforce server-side authorization on every request, not just in the UI.
- Least privilege with centralized policy code; never trust client-supplied IDs — check object ownership (per-user/per-tenant scoping).
- Disable directory listing and default accounts; log access-control failures and alert on repeated ones.

Source: OWASP Top 10:2021 — https://owasp.org/Top10/
