---
id = "std/security-plus/owasp-a05-security-misconfiguration"
kind = "protocol"
title = "OWASP A05:2021 — Security Misconfiguration"
category = "reference/security/security-plus/owasp"
tags = ["owasp", "a05", "misconfiguration", "hardening"]
version = "OWASP Top 10:2021"
---
Security misconfiguration covers insecure defaults and unhardened stacks: verbose errors, open cloud storage, unneeded ports and services, permissive CORS, stale frameworks.

## Typical example

```text
S3 bucket "acme-backups" left with public-read ACL  ->  entire archive browsable by anyone
```

## Defenses

- Deploy hardened, minimal baselines via infrastructure-as-code; strip default accounts, sample apps and debug endpoints from production.
- Automate configuration review in CI/CD; segment networks; show generic errors to users, log the details internally.

Source: OWASP Top 10:2021 — https://owasp.org/Top10/
