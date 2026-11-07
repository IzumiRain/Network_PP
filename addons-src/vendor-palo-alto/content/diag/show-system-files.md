---
id = "cmd/paloalto/show-system-files"
kind = "command"
title = "show system files"
category = "reference/vendors/palo-alto"
tags = ["palo-alto", "pan-os", "cli", "disk", "diagnostics"]
risk = "info"
version = "PAN-OS 10.x/11.x"
---
Lists files under /var/log with sizes: tech-support files, core dumps, pcaps.

## Command

```text
show system files
```

## Notes

- Quick check of what is consuming disk: old tech-support files and core dumps add up.
- Verify free space before software upgrades or taking new captures.
- Related view: `show system disk-space` for overall partition usage.

Source: Palo Alto PAN-OS Administrator's Guide — https://docs.paloaltonetworks.com
