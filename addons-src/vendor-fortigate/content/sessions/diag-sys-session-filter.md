---
id = "cmd/fortigate/diag-sys-session-filter"
kind = "command"
title = "diagnose sys session filter"
category = "reference/firewalls/fortigate/sessions"
tags = ["fortigate", "fortios", "cli", "sessions"]
risk = "low"
version = "FortiOS 7.4"
---
Sets the match criteria used by diagnose sys session list (and session clear); multiple filters combine with AND.

## Command

```cisco
diagnose sys session filter dst 10.20.0.15
diagnose sys session filter dport 443
diagnose sys session filter proto 6
diagnose sys session filter clear
```

## Notes

- Filters apply to the CLI session only and persist until cleared — clear them before each new investigation.
- Common keys: dst/src, dport/sport, proto, policy; an empty filter set matches every session.
- Changing the filter does not touch firewall configuration, only what the CLI displays.

Source: Fortinet FortiOS CLI Reference — https://docs.fortinet.com
