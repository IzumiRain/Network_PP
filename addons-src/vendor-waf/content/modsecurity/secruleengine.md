---
id = "ref/waf/secruleengine-directive"
kind = "configuration"
title = "SecRuleEngine directive"
category = "reference/security/waf/modsecurity"
tags = ["waf", "modsecurity", "directives"]
version = "ModSecurity 2.9 / v3"
---
Master switch deciding whether rules execute disruptively, log only, or not at all.

## Example

```apache
SecRuleEngine On        # On = enforce, DetectionOnly = match+log only, Off = disabled
```

## Notes

- `On` is the enforcing mode; `DetectionOnly` runs the full rule set but never disrupts - ideal for the first weeks.
- `Off` disables rule processing entirely; audit logging of matched rules also stops.
- Deploy new rule sets in DetectionOnly, review matched rules in the audit log, then switch to `On` deliberately.
- The setting is global; per-transaction overrides use `ctl:ruleEngine` actions, not a second SecRuleEngine.

Source: ModSecurity Reference Manual (OWASP) — https://owasp.org
