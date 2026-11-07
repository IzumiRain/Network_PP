---
id = "std/security-plus/owasp-a03-injection"
kind = "protocol"
title = "OWASP A03:2021 — Injection"
category = "reference/security/security-plus/owasp"
tags = ["owasp", "a03", "injection", "sqli", "xss"]
version = "OWASP Top 10:2021"
---
Injection occurs when untrusted data is sent to an interpreter as part of a command or query — SQL/NoSQL, OS command, LDAP, and (since the 2021 list) cross-site scripting.

## Typical example

```sql
SELECT * FROM users WHERE name = '' OR '1'='1' -- '   -- attacker input flips the query
```

## Defenses

- Parameterized queries/prepared statements and safe APIs; never build queries by string concatenation.
- Server-side allow-list input validation; escape per output context; use LIMIT and SQL controls to cap damage.
- Automated SAST/DAST scanning in CI; XSS prevention (output encoding, CSP) is handled here too.

Source: OWASP Top 10:2021 — https://owasp.org/Top10/
