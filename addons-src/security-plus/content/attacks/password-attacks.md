---
id = "std/security-plus/password-attacks"
kind = "error"
title = "Password attacks: spray, brute force, dictionary"
category = "reference/security/security-plus/attacks"
tags = ["security-plus", "password", "brute-force", "spray", "credential-stuffing"]
version = "SY0-701"
---
Password attacks differ in what they guess and how loudly; spraying is stealthy, brute force is noisy.

## Patterns

```text
brute force  try every combination            dictionary  try a wordlist of likely passwords
spraying     one weak password across many accounts, slowly, to dodge lockouts
stuffing     replay username/password pairs from other sites' breaches
```

## Notes

- Spraying avoids account lockouts because each account gets only a few tries per cycle (e.g., "Season2024!" rotated slowly).
- Pass-the-hash replays an NTLM hash directly without cracking the password (Windows environments).
- Defenses: MFA, lockout and throttling, long passphrases, breached-password screening, alert on failures spread across many accounts.

Source: CompTIA Security+ SY0-701 Exam Objectives — https://www.comptia.org
