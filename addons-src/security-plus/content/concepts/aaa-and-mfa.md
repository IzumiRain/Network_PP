---
id = "std/security-plus/aaa-and-mfa"
kind = "protocol"
title = "AAA framework and MFA types"
category = "reference/security/security-plus/concepts"
tags = ["security-plus", "aaa", "mfa", "authentication"]
version = "SY0-701"
---
AAA frames identity services as authentication, authorization and accounting; MFA strengthens the authentication step with factors from different categories.

## Summary

```text
AAA:  Authentication (who are you?) -> Authorization (what may you do?) -> Accounting (what did you do?)
MFA:  something you know + have + are (+ somewhere you are / something you do)
```

## Notes

- Factor examples: know = password/PIN; have = smart card, TOTP token, hardware key; are = fingerprint, face.
- Two factors from the same category (password + PIN) are NOT MFA — the categories must differ.
- Accounting is the logging/auditing leg; RADIUS and TACACS+ provide AAA for network access.
- MFA fatigue (push bombing) harasses users into approving prompts; number matching and hardware keys resist it.

Source: CompTIA Security+ SY0-701 Exam Objectives — https://www.comptia.org
