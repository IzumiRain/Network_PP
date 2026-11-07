---
id = "std/security-plus/ioc-vs-ioa"
kind = "protocol"
title = "Indicators of compromise (IoC) vs indicators of attack (IoA)"
category = "reference/security/security-plus/signatures"
tags = ["security-plus", "ioc", "ioa", "hash", "threat-intel"]
version = "SY0-701"
---
An IoC is forensic evidence that a compromise already happened; an IoA is a behavioral signal that an attack is underway right now.

## Example

```text
IoC: file SHA-256 hash on a watchlist, C2 IP/domain, registry key, unusual DNS traffic
IoA: powershell spawns -> writes to startup folder -> beacons out (sequence in progress)
```

## Notes

- Common IoC types: file hashes (MD5/SHA-256), IP addresses, domains, URLs, file names, mutexes.
- Hash-based indicators: EDR/AV continuously compute file digests and match them against known-bad watchlists — exact match only, so one recompiled byte defeats the hash.
- IoA focuses on attacker techniques in progress (lateral movement, privilege escalation, log clearing) rather than a single artifact.
- IoCs feed retrospective hunting (where else did this artifact appear); IoAs feed real-time detection and response.

Source: CISA guidance on indicators of compromise — https://www.cisa.gov
