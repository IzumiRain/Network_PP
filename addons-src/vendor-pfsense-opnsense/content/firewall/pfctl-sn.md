---
id = "cmd/pfsense/pfctl-sn"
kind = "command"
title = "pfctl -sn (show NAT rules)"
category = "reference/firewalls/pfsense-opnsense"
tags = ["pfsense", "opnsense", "pf", "freebsd", "cli", "nat"]
risk = "info"
version = "pfSense CE 2.7"
---
Shows the loaded NAT rules: outbound, port forwards and 1:1 mappings.

## Command

```bash
pfctl -sn
```

## Notes

- Shows NAT as pf sees it after translation of the WebGUI config (binat/rdr/nat rules).
- Port forwards appear as `rdr` rules on the interface they are bound to.
- When a port forward does not work, verify it exists here before touching the GUI.

Source: Netgate pfSense Documentation — https://docs.netgate.com
