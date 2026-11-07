---
id = "std/security-plus/owasp-a02-cryptographic-failures"
kind = "protocol"
title = "OWASP A02:2021 — Cryptographic Failures"
category = "reference/security/security-plus/owasp"
tags = ["owasp", "a02", "cryptography", "tls", "encryption"]
version = "OWASP Top 10:2021"
---
Cryptographic failures cover data left unprotected or protected with weak crypto: no TLS, legacy ciphers, hardcoded keys, or passwords hashed with broken algorithms.

## Typical example

```text
user table stores password hashes as unsalted MD5  ->  cracked offline in seconds after a breach
```

## Defenses

- TLS 1.2+ (prefer 1.3) everywhere with HSTS; classify data and encrypt in transit and at rest.
- Strong modern algorithms: AES-GCM for data, Argon2/bcrypt/scrypt/PBKDF2 for password hashing; never invent your own crypto.
- Manage keys properly — generation, storage in a vault/HSM, rotation; no keys or secrets in code or repos.

Source: OWASP Top 10:2021 — https://owasp.org/Top10/
