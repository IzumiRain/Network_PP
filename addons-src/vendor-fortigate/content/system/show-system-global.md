---
id = "cmd/fortigate/show-system-global"
kind = "command"
title = "show system global"
category = "reference/firewalls/fortigate/system"
tags = ["fortigate", "fortios", "cli", "system"]
risk = "info"
version = "FortiOS 7.4"
---
Dumps the running global system configuration: hostname, admin ports/timeouts, DNS, NTP and aliases.

## Command

```cisco
show system global
```

## Notes

- Check admintimeout and the admin-sport (HTTPS port) here before scripted sessions or GUI access fail mysteriously.
- Shows dns, ntp and central-management settings that affect lookups and updates fleet-wide.
- Read-only view of the config node; changes are made under config system global ... end.

Source: Fortinet FortiOS Administration Guide — https://docs.fortinet.com
