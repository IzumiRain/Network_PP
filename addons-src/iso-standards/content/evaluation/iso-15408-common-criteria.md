---
id = "std/iso-standards/iso-15408-common-criteria"
kind = "standard"
title = "ISO/IEC 15408 — Common Criteria (EAL levels)"
category = "reference/security/iso-standards/evaluation"
tags = ["iso15408", "common-criteria", "eal", "product-evaluation"]
version = "ISO/IEC 15408 / CC v3.1"
---
ISO/IEC 15408, known as Common Criteria (CC), certifies security PRODUCTS (not organizations) through licensed labs against assurance levels EAL1-EAL7.

## Evaluation Assurance Levels

```text
EAL1 functionally tested                             EAL5 semi-formally designed and tested
EAL2 structurally tested                             EAL6 semi-formally verified design and tested
EAL3 methodically tested and checked                 EAL7 formally verified design and tested
EAL4 methodically designed, tested and reviewed
```

## Notes

- Who needs it: vendors certifying firewalls, HSMs, smart cards and operating systems; governments whose procurement demands CC certificates.
- Mechanics: the vendor declares a Security Target, an accredited lab evaluates the target of evaluation (TOE), a national scheme issues the certificate; results are mutually recognized under the CCRA.
- Higher EAL means deeper assurance of design and implementation, not "more secure features".
- Relation: product-level complement to the process-level ISO/IEC 27001 ISMS.

Source: ISO — ISO/IEC 15408 Common Criteria — https://www.iso.org
