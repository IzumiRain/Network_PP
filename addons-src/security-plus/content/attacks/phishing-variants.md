---
id = "std/security-plus/phishing-variants"
kind = "error"
title = "Phishing variants"
category = "reference/security/security-plus/attacks"
tags = ["security-plus", "phishing", "spear-phishing", "smishing", "social-engineering"]
version = "SY0-701"
---
Phishing is social engineering delivered per channel; the variants are named by target and medium.

## Variants

```text
phishing      bulk email, wide net             spear phishing  targeted individual with research
whaling       targets executives               vishing         voice calls (fake IT/helpdesk)
smishing      SMS/text lures                   pharming        poisoned DNS sends victims to fakes
```

## Notes

- Common markers: urgency, claimed authority, mismatched sender domain, look-alike URLs, credential-harvest links or attachments.
- Business email compromise uses realistic internal threads with wire/gift-card requests instead of links.
- Defenses: SPF/DKIM/DMARC, attachment sandboxing, user training plus a one-click report button, MFA so stolen passwords alone are not enough.

Source: CISA phishing guidance — https://www.cisa.gov
