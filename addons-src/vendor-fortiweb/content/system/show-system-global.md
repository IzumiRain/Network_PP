---
id = "cmd/fortiweb/show-system-global"
kind = "command"
title = "show system global"
category = "reference/firewalls/fortiweb/system"
tags = ["fortiweb", "waf", "cli"]
risk = "info"
version = "FortiWeb 7.2"
---
Dumps the global system configuration: hostname, admin settings, DNS and related platform options.

## Command

```cisco
show system global
```

## Notes

- Verify DNS and admin access settings here before investigating signature updates or GUI access.
- Shows the admin HTTPS port and timeouts that scripted sessions depend on.
- Read-only view; changes are made under config system global.

Source: Fortinet FortiWeb Administration Guide — https://docs.fortinet.com
