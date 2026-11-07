---
id = "std/security-plus/owasp-a10-ssrf"
kind = "protocol"
title = "OWASP A10:2021 — Server-Side Request Forgery (SSRF)"
category = "reference/security/security-plus/owasp"
tags = ["owasp", "a10", "ssrf", "metadata"]
version = "OWASP Top 10:2021"
---
SSRF tricks a server into fetching an attacker-chosen URL, exposing internal services and cloud metadata endpoints that are unreachable from outside.

## Typical example

```text
GET /fetch?url=http://169.254.169.254/latest/meta-data/  ->  cloud instance credentials returned to attacker
```

## Defenses

- Allow-list permitted destinations and schemes; validate every URL the server will request.
- Block internal address ranges (link-local, RFC 1918) and disable HTTP redirects on outbound fetches.
- Segment the metadata endpoint and never return raw response bodies to the client.

Source: OWASP Top 10:2021 — https://owasp.org/Top10/
