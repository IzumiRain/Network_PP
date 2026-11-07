---
id = "cmd/fortigate/diag-sys-session-list"
kind = "command"
title = "diagnose sys session list"
category = "reference/firewalls/fortigate/sessions"
tags = ["fortigate", "fortios", "cli", "sessions"]
risk = "info"
version = "FortiOS 7.4"
---
Dumps the live session table: endpoints, protocol and state, NAT translation, expiry and the matching policy id.

## Command

```cisco
diagnose sys session list
```

## Notes

- Always narrow with diagnose sys session filter first — the full table is huge on a busy firewall.
- The policy id field tells you which firewall rule the flow actually hit.
- NAT fields show the translated address/port, useful to verify SNAT/DNAT behavior.

Source: Fortinet FortiOS CLI Reference — https://docs.fortinet.com
