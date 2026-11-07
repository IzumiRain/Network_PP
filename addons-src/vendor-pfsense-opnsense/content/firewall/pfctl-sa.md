---
id = "cmd/pfsense/pfctl-sa"
kind = "command"
title = "pfctl -sa (show everything)"
category = "reference/vendors/pfsense-opnsense"
tags = ["pfsense", "opnsense", "pf", "freebsd", "cli"]
risk = "info"
version = "pfSense CE 2.7"
---
Prints all pf information: rules, NAT, states, tables and statistics.

## Command

```bash
pfctl -sa
```

## Notes

- Equivalent of running every `-s` view at once; output is long, so pipe through grep.
- Useful as a single snapshot to attach to a ticket before and after a change.
- The states and tables sections are the ones that grow with traffic.
- Read-only: safe on production.

Source: Netgate pfSense Documentation — https://docs.netgate.com
