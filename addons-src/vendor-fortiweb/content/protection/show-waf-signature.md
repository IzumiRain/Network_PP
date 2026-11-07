---
id = "cmd/fortiweb/show-waf-signature"
kind = "command"
title = "show waf signature"
category = "reference/firewalls/fortiweb/protection"
tags = ["fortiweb", "waf", "cli", "signatures"]
risk = "info"
version = "FortiWeb 7.2"
---
Lists the custom WAF signatures defined on the appliance, with their match conditions and actions.

## Command

```cisco
show waf signature
```

## Notes

- Custom signatures live under config waf signature and are combined into signature groups for a profile.
- Use it to audit what site-specific rules exist before tuning or deleting anything.
- Built-in FortiGuard signatures are separate and updated by subscription; this shows only your own rules.

Source: Fortinet FortiWeb Administration Guide — https://docs.fortinet.com
