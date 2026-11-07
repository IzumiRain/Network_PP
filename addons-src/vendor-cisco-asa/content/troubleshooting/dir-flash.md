---
id = "cmd/asa/dir-flash"
kind = "command"
title = "dir flash: (list files)"
category = "reference/vendors/cisco/asa/troubleshooting"
tags = ["asa", "cisco", "cli", "files"]
risk = "info"
version = "ASA 9.16"
---
Lists files on the internal flash (disk0:) - images, ASDM, crashinfo, backups.

## Command

```cisco
dir flash:
dir flash:/log
show flash:
```

## Notes

- `flash:` and `disk0:` refer to the same storage on ASA platforms; `show flash:` is an equivalent alias.
- Verify the exact image filename here before changing the boot variable or upgrading.
- Crashinfo files live under flash:/log - a growing pile of crash dumps signals repeated reloads worth investigating.
- Check free space before `copy` operations; a full flash blocks image and config uploads.

Source: Cisco ASA Series Command Reference — https://www.cisco.com
