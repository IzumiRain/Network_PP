---
id = "ref/waf/transaction-phases"
kind = "protocol"
title = "ModSecurity transaction phases"
category = "reference/security/waf/modsecurity"
tags = ["waf", "modsecurity", "phases"]
version = "ModSecurity 2.9 / v3"
---
Each request is processed through five phases; rule phase decides what data is available.

## Phases

| Phase | Stage | Typical use |
|---|---|---|
| 1 | Request headers | URI/method/header checks, early deny |
| 2 | Request body | ARGS inspection, SQLi/XSS detection |
| 3 | Response headers | Outbound header checks |
| 4 | Response body | Outbound leakage/defacement checks |
| 5 | Logging | Custom audit and scoring logic |

## Notes

- Body inspection requires request body access enabled: `SecRequestBodyAccess On` (and `SecResponseBodyAccess On` for phases 3-4).
- A deny in phase 1 or 2 stops the transaction before the backend sees it; response phases can only log or block the response.
- Rules using variables that do not exist in their phase simply never match - a common cause of "my rule never fires".

Source: ModSecurity Reference Manual (OWASP) — https://owasp.org
