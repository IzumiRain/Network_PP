---
id = "ref/waf/virtual-patching"
kind = "protocol"
title = "Virtual patching"
category = "reference/security/waf/concepts"
tags = ["waf", "concepts", "virtual-patching"]
version = "WAF concepts (ModSecurity 2.x/3.x)"
---
Deploying a WAF rule that blocks exploitation of a specific vulnerability before the application itself is fixed.

## Example

```apache
# Temporary virtual patch while the app team fixes CVE-2024-0001
SecRule REQUEST_URI "@rx ^/api/legacy/transfer" \
  "id:900100,phase:1,deny,log,msg:'Virtual patch: CVE-2024-0001 endpoint blocked'"
```

## Notes

- Buys time between disclosure and the real code fix; it is a mitigation, never a replacement for patching.
- Scope the rule as tightly as possible (URI, parameter, source) or it becomes a permanent source of false positives.
- Tag virtual patches (e.g. `tag:'vpatch-cve-2024-0001'`) and track expiry so they get removed after the fix ships.
- The OWASP CRS project and many vendors publish pre-made virtual-patch rule packs for active CVEs.

Source: OWASP WAF / AppSec documentation — https://owasp.org
