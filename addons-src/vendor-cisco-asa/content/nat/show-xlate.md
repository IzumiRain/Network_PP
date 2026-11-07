---
id = "cmd/asa/show-xlate"
kind = "command"
title = "show xlate"
category = "reference/firewalls/cisco-asa/nat"
tags = ["asa", "cisco", "cli", "nat"]
risk = "info"
version = "ASA 9.16"
---
Shows the active NAT translation (xlate) table.

## Command

```cisco
show xlate
show xlate detail
```

## Notes

- Confirms a translation exists for a live flow; missing xlate often means the NAT rule never matched.
- Use `show xlate local 10.1.1.25` (or `interface inside`) to find what an internal host is translated to.
- `clear xlate` tears down existing translations and drops their flows - disruptive, never run casually.
- Stale xlates normally expire with idle timeouts; many leftover statics after config churn may need cleanup.

Source: Cisco ASA Series Command Reference — https://www.cisco.com
