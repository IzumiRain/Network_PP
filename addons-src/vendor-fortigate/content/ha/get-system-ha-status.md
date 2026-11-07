---
id = "cmd/fortigate/get-system-ha-status"
kind = "command"
title = "get system ha status"
category = "reference/firewalls/fortigate/ha"
tags = ["fortigate", "fortios", "cli", "ha"]
risk = "info"
version = "FortiOS 7.4"
---
Shows the HA cluster state: mode, group id, primary/secondary roles, priorities, serials and sync status per member.

## Command

```cisco
get system ha status
```

## Notes

- Always confirm which member is primary here before any maintenance or reboot in an HA pair.
- The member list shows priority, uptime and serial number; mismatches hint at split-brain or stale units.
- Pair with show system ha to see the full HA configuration (mode, override, heartbeat interfaces).

Source: Fortinet FortiOS Administration Guide — https://docs.fortinet.com
