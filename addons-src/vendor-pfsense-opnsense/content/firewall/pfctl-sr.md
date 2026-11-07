---
id = "cmd/pfsense/pfctl-sr"
kind = "command"
title = "pfctl -sr (show rules)"
category = "reference/firewalls/pfsense-opnsense"
tags = ["pfsense", "opnsense", "pf", "freebsd", "cli"]
risk = "info"
version = "pfSense CE 2.7"
---
Shows the packet-filter ruleset exactly as pf has it loaded.

## Command

```bash
pfctl -sr
```

## Notes

- Shows what pf actually enforces, including WebGUI-generated rules wrapped in anchors.
- Compare with `pfctl -sn` for NAT rules and `pfctl -ss` for active states.
- Rules added by the GUI often appear as anchor sub-rulesets rather than flat lines.
- Read-only: safe on production.

Source: Netgate pfSense Documentation — https://docs.netgate.com
