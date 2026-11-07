---
id = "cmd/pfsense/filter-log"
kind = "command"
title = "read the firewall (filter) log"
category = "reference/firewalls/pfsense-opnsense"
tags = ["pfsense", "opnsense", "cli", "logs"]
risk = "info"
version = "pfSense CE 2.7 / OPNsense 24.x"
---
Reads the firewall packet log from the shell; paths differ by product and version.

## Command

```bash
# pfSense CE 2.4 and earlier
clog /var/log/filter.log
# pfSense CE 2.5 and later
clog /var/log/filter/filter.log
# OPNsense
tail -f /var/log/firewall/latest.log
```

## Notes

- pfSense logs are circular ring buffers; clog reads the live file without decompression.
- Since pfSense 2.5 rotated logs live in /var/log/filter/ as .gz files.
- OPNsense keeps the firewall log in /var/log/firewall/ as plain text, so tail/grep work directly.
- Log lines show time, interface, action (block/pass), protocol and src -> dst.

Source: Netgate pfSense Documentation — https://docs.netgate.com
