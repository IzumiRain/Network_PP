---
id = "std/security-plus/owasp-a06-vulnerable-components"
kind = "protocol"
title = "OWASP A06:2021 — Vulnerable and Outdated Components"
category = "reference/security/security-plus/owasp"
tags = ["owasp", "a06", "dependencies", "cve", "patching"]
version = "OWASP Top 10:2021"
---
Running libraries, frameworks or platforms with known CVEs — or end-of-life software nobody patches — turns public exploits into direct entry points.

## Typical example

```text
web tier ships a framework version with a public RCE CVE  ->  unauthenticated remote code execution
```

## Defenses

- Maintain a software inventory/SBOM including transitive dependencies; remove unused ones.
- Continuous dependency scanning (SCA) plus CVE feed monitoring (NVD); prioritize by exploitability and exposure.
- Patch management with an SLA; watch end-of-life announcements and migrate off dead frameworks.

Source: OWASP Top 10:2021 — https://owasp.org/Top10/
