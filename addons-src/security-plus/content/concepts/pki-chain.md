---
id = "std/security-plus/pki-chain"
kind = "protocol"
title = "PKI certificate chain"
category = "reference/security/security-plus/concepts"
tags = ["security-plus", "pki", "ca", "x509", "certificates"]
version = "SY0-701"
---
A PKI delegates trust down a certificate chain: an offline root CA signs intermediate CAs, which sign end-entity certificates.

## Chain

```text
Root CA (offline, self-signed, in the trust store) -> Intermediate CA -> leaf cert for www.example.com
```

## Notes

- A CSR (public key + identity) is signed by the CA into an X.509 certificate; clients validate the chain up to a trust anchor in the OS/browser store.
- The root CA stays offline; intermediates do day-to-day signing so the root key is never exposed.
- Revocation: CRL (periodic list) or OCSP (live query); expired or revoked certificates fail validation.
- One broken intermediate invalidates everything it signed — keep the root offline and key ceremonies audited.

Source: CompTIA Security+ SY0-701 Exam Objectives — https://www.comptia.org
