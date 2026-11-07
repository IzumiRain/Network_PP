---
id = "ref/waf/sqli-detection-example"
kind = "configuration"
title = "SQL injection detection rule example"
category = "reference/security/waf/rules"
tags = ["waf", "modsecurity", "sqli", "rules"]
version = "ModSecurity 2.9 / CRS 3.x/4.x"
---
A minimal SQLi detection rule using libinjection, the same engine family CRS production rules build on.

## Example

```apache
SecRule ARGS|ARGS_NAMES "@detectSQLi" \
  "id:1004,phase:2,t:none,t:lowercase,t:urlDecodeUni,deny,log,tag:'custom-sqli',msg:'SQL injection attempt'"
```

## Notes

- `@detectSQLi` fingerprints SQLi payloads via libinjection; `@detectXSS` is its XSS counterpart.
- The CRS production equivalent lives in the 942-SQLI files (e.g. rule 942100) with many layered regexes and transformations.
- Keep the transformation chain (lowercase, urlDecodeUni) or encoded payloads pass through undetected.
- Under anomaly scoring, replace `deny` with a score increment and let the threshold rules make the final decision.

Source: OWASP Core Rule Set Documentation — https://coreruleset.org
