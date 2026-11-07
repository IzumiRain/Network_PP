---
id = "ref/waf/xss-detection-example"
kind = "configuration"
title = "XSS detection rule example"
category = "reference/security/waf/rules"
tags = ["waf", "modsecurity", "xss", "rules"]
version = "ModSecurity 2.9 / CRS 3.x/4.x"
---
A minimal XSS detection rule over arguments and referer header using libinjection.

## Example

```apache
SecRule ARGS|REQUEST_HEADERS:Referer "@detectXSS" \
  "id:1005,phase:2,t:none,t:lowercase,t:urlDecodeUni,deny,log,tag:'custom-xss',msg:'XSS attempt'"
```

## Notes

- `@detectXSS` recognizes common script/HTML injection fingerprints in any targeted variable.
- The CRS production equivalents are the 941-XSS rules (941100 onward), which add regex layers and HTML-entity handling.
- Response-body XSS is not covered by request-side rules; CRS outbound files (950s) handle response leakage.
- In DetectionOnly deployments this rule shape is the safest first test for request-body parsing.

Source: OWASP Core Rule Set Documentation — https://coreruleset.org
