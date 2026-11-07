---
id = "cmd/pfsense/pfctl-si"
kind = "command"
title = "pfctl -si (show info and stats)"
category = "reference/firewalls/pfsense-opnsense"
tags = ["pfsense", "opnsense", "pf", "freebsd", "cli"]
risk = "info"
version = "pfSense CE 2.7"
---
Shows pf counters: state table usage, searches, matches, blocks and passes.

## Command

```bash
pfctl -si
```

## Notes

- Watch `current entries` against the state-table limit when connections fail.
- Rising block counters confirm that pf is actively dropping something.
- Good for a quick health check without dumping the whole ruleset.

Source: Netgate pfSense Documentation — https://docs.netgate.com
