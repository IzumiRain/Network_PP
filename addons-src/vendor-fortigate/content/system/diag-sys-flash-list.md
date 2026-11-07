---
id = "cmd/fortigate/diag-sys-flash-list"
kind = "command"
title = "diagnose sys flash list"
category = "reference/firewalls/fortigate/system"
tags = ["fortigate", "fortios", "cli", "firmware"]
risk = "info"
version = "FortiOS 7.4"
---
Lists the flash partitions and installed firmware images with their boot partition assignment.

## Command

```cisco
diagnose sys flash list
```

## Notes

- Shows partition names, sizes, and which image is marked as the current boot image.
- Use it after an upgrade to confirm which firmware actually booted, and before a downgrade to check free space.
- Read-only; complements the Version line from get system status.

Source: Fortinet FortiOS CLI Reference — https://docs.fortinet.com
