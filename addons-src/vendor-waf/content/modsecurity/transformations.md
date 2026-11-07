---
id = "ref/waf/transformation-functions"
kind = "configuration"
title = "Transformation functions (t: actions)"
category = "reference/security/waf/modsecurity"
tags = ["waf", "modsecurity", "transformations"]
version = "ModSecurity 2.9 / v3"
---
Transformations normalize variable values before the operator runs, closing encoding bypasses.

## Example

```apache
SecRule ARGS "@rx select" \
  "id:1003,phase:2,t:none,t:lowercase,t:urlDecodeUni,deny,log,msg:'SQLi keyword'"
```

## Notes

- Common functions: `t:lowercase`, `t:urlDecodeUni`, `t:htmlEntityDecode`, `t:removeNulls`, `t:replaceComments`.
- Transformations chain in written order and the operator sees only the final transformed value.
- `t:none` clears transformations inherited from earlier rules in the chain.
- CRS rules list several transformations per rule; when writing exclusions or custom rules, mirror the same chain or detection can be bypassed by encoding.

Source: ModSecurity Reference Manual (OWASP) — https://owasp.org
