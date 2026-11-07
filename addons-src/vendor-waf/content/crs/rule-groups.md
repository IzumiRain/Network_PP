---
id = "ref/waf/crs-rule-groups"
kind = "protocol"
title = "CRS rule groups (rule file map)"
category = "reference/security/waf/crs"
tags = ["waf", "crs", "rule-ids"]
version = "OWASP CRS 3.x/4.x"
---
CRS rules live in numbered files under the rules/ directory; rule IDs share their file's numeric prefix.

## Key groups

| Prefix | Rule file | Purpose |
|---|---|---|
| 920 | 920-PROTOCOL-ENFORCEMENT | HTTP conformance (methods, headers, versions) |
| 921 | 921-PROTOCOL-ATTACK | Request smuggling, splitting, poisoning |
| 930-933 | LFI / RFI / RCE / PHP files | Path, injection and code-execution families |
| 941 | 941-XSS | Cross-site scripting |
| 942 | 942-SQLI | SQL injection |
| 943 | 943-SESSION-FIXATION | Session fixation |
| 944 | 944-RCE-JAVA | Java deserialization / Java RCE |

## Notes

- When the audit log reports rule id 942xxxx, the tuning targets the 942-SQLI file - the prefix maps 1:1.
- Inbound files run in phases 1-2; response-side rules (959/950 files) evaluate outbound content.
- Load CRS by including the CRS setup then the rule files; individual files can be excluded from the include list if a family does not apply.

Source: OWASP Core Rule Set Documentation — https://coreruleset.org
