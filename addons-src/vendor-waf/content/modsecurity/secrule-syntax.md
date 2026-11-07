---
id = "ref/waf/secrule-syntax"
kind = "configuration"
title = "SecRule syntax basics"
category = "reference/security/waf/modsecurity"
tags = ["waf", "modsecurity", "directives"]
version = "ModSecurity 2.9 / v3"
---
Every CRS and custom rule is a SecRule: variables, an operator, and an action list.

## Example

```apache
SecRule REQUEST_URI "@rx /admin" \
  "id:1002,phase:1,deny,log,status:403,msg:'Admin path touched'"
```

## Notes

- Structure: `SecRule <variables> "<operator>" "<actions>"` - e.g. variables like `ARGS`, `REQUEST_HEADERS`, `REQUEST_URI`.
- `id` is mandatory and must be unique; CRS uses 9xxxxx ranges, pick a private range (1xxxxxx) for custom rules.
- `phase:1|2` targets request headers/body, `phase:3|4` response, `phase:5` logging.
- Disruptive actions (`deny`, `status`, `allow`) only take effect when SecRuleEngine is On; `log`/`nolog` and `msg`/`tag` shape the audit trail.

Source: ModSecurity Reference Manual (OWASP) — https://owasp.org
