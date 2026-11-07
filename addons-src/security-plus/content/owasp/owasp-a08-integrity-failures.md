---
id = "std/security-plus/owasp-a08-integrity-failures"
kind = "protocol"
title = "OWASP A08:2021 — Software and Data Integrity Failures"
category = "reference/security/security-plus/owasp"
tags = ["owasp", "a08", "supply-chain", "deserialization", "ci-cd"]
version = "OWASP Top 10:2021"
---
Software and data integrity failures happen when code or data from untrusted sources is accepted without verification: unsigned updates, unverified dependencies, insecure deserialization.

## Typical example

```text
CI pipeline pulls an update package with no signature check  ->  trojanized build ships to customers
```

## Defenses

- Verify digital signatures/checksums of dependencies and updates; sign artifacts; pin dependency versions from trusted repositories.
- Never deserialize untrusted data; if unavoidable, enforce integrity checks and strict type allow-lists.
- Protect CI/CD: least-privilege pipeline credentials, reviewed changes, audited pipeline actions.

Source: OWASP Top 10:2021 — https://owasp.org/Top10/
