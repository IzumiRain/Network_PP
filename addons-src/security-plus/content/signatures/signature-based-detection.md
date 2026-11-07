---
id = "std/security-plus/signature-based-detection"
kind = "protocol"
title = "Signature-based IDS/IPS detection"
category = "reference/security/security-plus/signatures"
tags = ["security-plus", "ids", "ips", "signatures", "detection"]
version = "SY0-701"
---
Signature-based detection matches traffic or files against a database of known-bad patterns (signatures) — the default mode of most IDS/IPS and antivirus engines.

## How it works

```text
capture packet/file -> match against signature database -> hit? -> IDS: alert  /  IPS: drop or reset
```

## Notes

- Strengths: precise, low false-positive rate, easy to tune and explain to auditors.
- Weaknesses: blind to zero-day, novel and obfuscated attacks; coverage is only as current as the signature feeds.
- IDS is passive (alerts on a copy via SPAN/TAP); IPS sits inline and enforces (drop, reset, block).
- Feeds must update constantly; a stale signature database silently degrades coverage.
- Exam tip: "known attacks only" is the giveaway phrase for signature-based detection.

Source: CompTIA Security+ SY0-701 Exam Objectives — https://www.comptia.org
