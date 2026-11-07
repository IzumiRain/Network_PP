---
id = "std/security-plus/owasp-a07-auth-failures"
kind = "protocol"
title = "OWASP A07:2021 — Identification and Authentication Failures"
category = "reference/security/security-plus/owasp"
tags = ["owasp", "a07", "authentication", "sessions", "mfa"]
version = "OWASP Top 10:2021"
---
Formerly "Broken Authentication": weaknesses in confirming identity and managing sessions permit credential stuffing, brute force and session hijacking.

## Typical example

```text
/login allows unlimited attempts, no MFA, session tokens never rotate  ->  stuffing succeeds and persists
```

## Defenses

- MFA for all users; rate-limit and monitor authentication attempts; screen against known-breached passwords.
- Session management: rotate session IDs after login, short idle and absolute timeouts, invalidate on logout, no predictable tokens.
- Align password policy with NIST SP 800-63 (length over complexity churn).

Source: OWASP Top 10:2021 — https://owasp.org/Top10/
