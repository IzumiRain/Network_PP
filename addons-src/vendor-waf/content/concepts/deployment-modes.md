---
id = "ref/waf/deployment-modes"
kind = "protocol"
title = "Deployment modes: reverse proxy vs embedded"
category = "reference/security/waf/concepts"
tags = ["waf", "concepts", "deployment"]
version = "WAF concepts (ModSecurity 2.x/3.x)"
---
A WAF runs either as a standalone reverse-proxy tier in front of the app, or embedded inside the web server that serves the app.

## Comparison

| Mode | Placement | Trade-off |
|---|---|---|
| Reverse proxy | Own server/VM/container in front of app servers | Clean scaling and TLS handling; extra hop and latency |
| Embedded | Module/connector in the app web server | No extra hop; consumes app server resources |

## Notes

- Reverse proxy deployments decouple WAF upgrades from application releases and shield topology and TLS from the internet.
- Embedded (Apache module or nginx connector) is the classic ModSecurity setup and sees the same traffic the app sees.
- With libmodsecurity v3 the WAF logic is a library and the deployment shape depends on the connector used.
- X-Forwarded-For handling and header normalization must be correct, or both modes log/block on wrong client IPs.

Source: OWASP WAF / AppSec documentation — https://owasp.org
