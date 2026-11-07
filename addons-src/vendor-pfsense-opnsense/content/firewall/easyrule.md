---
id = "cmd/pfsense/easyrule"
kind = "command"
title = "easyrule (quick block/pass from CLI)"
category = "reference/vendors/pfsense-opnsense"
tags = ["pfsense", "cli", "firewall"]
risk = "medium"
version = "pfSense CE 2.7"
---
Adds a real block or pass rule to the running configuration from the shell.

## Command

```bash
easyrule block wan 203.0.113.50
easyrule pass lan 10.0.1.50 8.8.8.8 443 tcp
```

## Notes

- `block` takes interface plus source; `pass` optionally adds destination, port and protocol.
- Rules created here are persistent and visible in the GUI under Firewall > Rules.
- Verify the result with `pfctl -sr` or in the WebGUI.
- Still follow the normal change process - this writes to the firewall config.

Source: Netgate pfSense Documentation — https://docs.netgate.com
