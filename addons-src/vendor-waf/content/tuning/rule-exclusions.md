---
id = "ref/waf/rule-exclusions"
kind = "configuration"
title = "False-positive tuning: rule exclusions"
category = "reference/security/waf/tuning"
tags = ["waf", "crs", "tuning", "false-positives"]
version = "OWASP CRS 3.x/4.x"
---
Exclude specific rules or rule targets for trusted endpoints instead of disabling rules globally.

## Example

```apache
# REQUEST-900-EXCLUSION-RULES-BEFORE-CRS.conf
# Remove one target (parameter) from one rule for one URI
SecRule REQUEST_URI "@beginsWith /api/search" \
  "id:1006,phase:1,pass,nolog,ctl:ruleRemoveTargetById=942100;ARGS:q"
# Global per-rule options
SecRuleRemoveById 920320
SecRuleUpdateTargetById 942100 "!ARGS:legacy_query"
```

## Notes

- Narrowest first: target-scoped removal (rule + variable, ideally URI-scoped) beats dropping a whole rule.
- The REQUEST-900 exclusion file is the supported place for CRS exclusions and ships with documented templates.
- `SecRuleRemoveById` disables a rule entirely - audit log the reason and review it periodically.
- Repeat offenders at higher paranoia levels usually need their own exclusion set, not a lower PL globally.

Source: OWASP Core Rule Set Documentation — https://coreruleset.org
