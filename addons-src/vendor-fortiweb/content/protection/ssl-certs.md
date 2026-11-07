---
id = "ref/fortiweb/ssl-certs"
kind = "configuration"
title = "SSL/TLS termination and certificates"
category = "reference/firewalls/fortiweb/protection"
tags = ["fortiweb", "waf", "ssl", "certificates"]
version = "FortiWeb 7.2"
---
FortiWeb terminates TLS in front of the web servers: HTTPS server policies present a server certificate, inspect plaintext HTTP, and re-encrypt towards the pool if configured.

## Key pieces

- Server certificate: imported (with key) or generated on the appliance, then bound to the HTTPS server policy.
- Trusted CAs: imported for validating upstream servers and, where used, client certificates.
- TLS settings: protocol versions and cipher suites per policy; older versions should be disabled.

## Notes

- A server policy mismatch between SNI and certificate causes client trust errors; test with a browser and a TLS scanner.
- Certificate imports and renewals are managed in the GUI certificate store; keep expiry dates in monitoring.
- After replacing a certificate, re-check the server policy binding before decommissioning the old one.

Source: Fortinet FortiWeb Administration Guide — https://docs.fortinet.com
