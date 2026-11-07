---
id = "std/security-plus/injection-families"
kind = "error"
title = "Injection attack families"
category = "reference/security/security-plus/attacks"
tags = ["security-plus", "injection", "sqli", "xss", "command-injection"]
version = "SY0-701"
---
Every injection attack is the same idea: untrusted input reaches an interpreter and is executed as code or query syntax.

## Families

```sql
' OR '1'='1 --      <- classic SQL injection closing a WHERE clause
```

```text
SQL/NoSQL injection     manipulate queries        command injection   run OS shell commands
XSS (reflected/stored/DOM) inject scripts in pages  LDAP injection      alter directory filters
XXE / XML injection     abuse entity resolution   template injection  abuse template engines
```

## Notes

- Defenses: parameterized queries/prepared statements, safe APIs, output encoding, server-side allow-list validation.
- A WAF is a compensating layer, not a fix — repair the code path.
- XSS is counted under injection in OWASP Top 10:2021 (A03).

Source: OWASP Injection Prevention Cheat Sheet — https://cheatsheetseries.owasp.org
