---
id = "cmd/fortiweb/show-waf-geo-ip-policy"
kind = "command"
title = "show waf geo-ip-policy"
category = "reference/firewalls/fortiweb/protection"
tags = ["fortiweb", "waf", "cli", "geo"]
risk = "info"
version = "FortiWeb 7.2"
---
Prints the geo-IP policies: country lists that are allowed or blocked for the protected sites.

## Command

```cisco
show waf geo-ip-policy
```

## Notes

- Typical pattern is allow-list mode for country-specific sites and block-list mode for global ones.
- Geo decisions run before most WAF inspection, so an overly wide block list cuts traffic early.
- Check the action (alert vs block) when users report being unable to reach the site from abroad.

Source: Fortinet FortiWeb Administration Guide — https://docs.fortinet.com
