---
id = "cmd/sophos/system-route-print"
kind = "command"
title = "system route print (SFOS)"
category = "reference/firewalls/sophos"
tags = ["sophos", "sfos", "cli", "routing"]
risk = "info"
version = "Sophos Firewall OS 19.x+"
---
Displays the SFOS routing table (connected, static and dynamic routes).

## Command

```text
system route print
```

## Notes

- Verify expected static routes here before debugging VPN or WAN failover issues.
- Subcommands in the `system route` family vary between SFOS releases - confirm the exact syntax in the CLI guide for your firmware version.
- The WebAdmin routing view shows the same data in table form.

Source: Sophos Firewall CLI Guide — https://docs.sophos.com
