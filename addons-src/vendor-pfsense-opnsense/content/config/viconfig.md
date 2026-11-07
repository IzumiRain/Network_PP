---
id = "cmd/pfsense/viconfig"
kind = "command"
title = "viconfig (edit config.xml)"
category = "reference/vendors/pfsense-opnsense"
tags = ["pfsense", "cli", "config"]
risk = "high"
version = "pfSense CE 2.7"
---
pfSense shell wrapper that opens /conf/config.xml directly in vi.

## Command

```bash
viconfig
```

## Notes

- Edits raw XML with no validation - a malformed file can keep the firewall from booting cleanly.
- Changes do not reach the running filter until the config is saved/applied via the GUI or a reboot.
- Make a copy first: `cp /conf/config.xml /root/config-before-edit.xml`.
- Prefer the WebGUI or API unless following vendor or TAC instructions step by step.

Source: Netgate pfSense Documentation — https://docs.netgate.com
