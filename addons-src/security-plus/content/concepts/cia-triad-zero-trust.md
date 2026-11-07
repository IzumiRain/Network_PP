---
id = "std/security-plus/cia-triad-and-zero-trust"
kind = "protocol"
title = "CIA triad and zero trust"
category = "reference/security/security-plus/concepts"
tags = ["security-plus", "cia", "zero-trust", "concepts"]
version = "SY0-701"
---
The CIA triad is the goal model of information security; zero trust is the modern architecture stance for pursuing it.

## Summary

```text
CIA triad:  Confidentiality (encryption, access control)  Integrity (hashes, signatures)  Availability (redundancy, backups)
Zero trust: verify explicitly -> least privilege -> assume breach
```

## Notes

- Confidentiality: only authorized eyes see data. Integrity: data is not altered undetected. Availability: systems usable when needed (DDoS is an availability attack).
- Zero trust (NIST SP 800-207): no implicit trust from network location — every request is authenticated, authorized and encrypted per session.
- Core zero-trust moves: micro-segmentation, centralized policy decisions, continuous monitoring, least-privilege access.
- Exam tip: "no implicit trust based on network location" is the zero-trust one-liner.

Source: NIST SP 800-207 Zero Trust Architecture — https://csrc.nist.gov
