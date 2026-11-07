---
id = "cmd/pfsense/pfctl-enable-disable"
kind = "command"
title = "pfctl -d / pfctl -e (disable and enable pf)"
category = "reference/firewalls/pfsense-opnsense"
tags = ["pfsense", "opnsense", "pf", "freebsd", "cli"]
risk = "high"
version = "pfSense CE 2.7"
---
Disables or re-enables the packet filter entirely.

## Command

```bash
pfctl -d
pfctl -e
```

## Notes

- `pfctl -d` turns off all firewalling and NAT until pf is enabled again.
- Use only for short, controlled diagnostics on networks you fully trust.
- A GUI filter reload re-enables pf automatically, which can end your test early.
- Re-enable with `pfctl -e` and verify with `pfctl -si` afterwards.

Source: Netgate pfSense Documentation — https://docs.netgate.com
