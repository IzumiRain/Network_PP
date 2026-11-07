---
id = "std/security-plus/heuristic-anomaly-behavioral-detection"
kind = "protocol"
title = "Heuristic, anomaly and behavioral detection"
category = "reference/security/security-plus/signatures"
tags = ["security-plus", "anomaly", "behavioral", "heuristic", "detection"]
version = "SY0-701"
---
Beyond exact signatures, detection engines use heuristics, statistical anomaly baselines and behavioral analysis to catch attacks nobody has written a rule for yet.

## Pattern

```text
heuristic   -> rules of thumb / trait scoring (catches malware variants, more false positives)
anomaly     -> statistical baseline of normal, alert on deviation (catches zero-days)
behavioral  -> sequences of actions over time (UEBA/EDR: process -> file -> network)
```

## Notes

- Heuristic inspection looks at code characteristics and behavior traits instead of exact byte patterns.
- Anomaly detection needs a learning/baselining period; alerts fired before the baseline settles are mostly noise.
- Anomaly and behavioral engines are the go-to for insider threats and slow lateral movement.
- Trade-off: fewer known-bad dependencies than signatures, but a higher false-positive rate.

Source: CompTIA Security+ SY0-701 Exam Objectives — https://www.comptia.org
