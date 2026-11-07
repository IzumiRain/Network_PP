---
id = "std/security-plus/owasp-a04-insecure-design"
kind = "protocol"
title = "OWASP A04:2021 — Insecure Design"
category = "reference/security/security-plus/owasp"
tags = ["owasp", "a04", "threat-modeling", "sdlc"]
version = "OWASP Top 10:2021"
---
Insecure design means the required security controls were never designed in — an architecture or flow flaw, distinct from implementation bugs (A03) or misconfiguration (A05).

## Typical example

```text
password recovery asks only for a birthdate and lets anyone reset any account  ->  flaw in the flow itself
```

## Defenses

- Establish a secure development lifecycle with threat modeling for critical flows (STRIDE, attack trees).
- Use secure design patterns and reference architectures; define trust boundaries deliberately.
- Add limits on resource use (rate limits, quotas) and design abuse cases alongside user stories.

Source: OWASP Top 10:2021 — https://owasp.org/Top10/
