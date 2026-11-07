---
id = "std/security-plus/owasp-a09-logging-failures"
kind = "protocol"
title = "OWASP A09:2021 — Security Logging and Monitoring Failures"
category = "reference/security/security-plus/owasp"
tags = ["owasp", "a09", "logging", "siem", "monitoring"]
version = "OWASP Top 10:2021"
---
Without logging and monitoring of security events, breaches go undetected for months and incident response has nothing to work with.

## Typical example

```text
10,000 failed logins in an hour from one source  ->  no alert configured, brute force quietly continues
```

## Defenses

- Log authentication, access-control and input-validation failures with context (user, IP, request); make logs tamper-evident.
- Centralize into a SIEM with alert thresholds and an incident response workflow; protect log integrity and retention.
- Test detectability: pen-test and purple-team exercises should prove attacks actually trigger alerts.

Source: OWASP Top 10:2021 — https://owasp.org/Top10/
