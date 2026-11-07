---
id = "cmd/mikrotik/backup-save"
kind = "command"
title = "/system backup save"
category = "reference/vendors/mikrotik"
tags = ["mikrotik", "routeros", "cli", "backup"]
risk = "low"
version = "RouterOS 7.x"
---
Creates a binary .backup file of the full router configuration on flash storage.

## Command

```text
/system backup save name=daily-backup
/system backup save name=daily-backup password=s3cret
```

## Notes

- Binary backups restore the exact configuration; best for identical hardware and RouterOS version.
- With `password=` the backup is encrypted; without it secrets are stored in the clear.
- Restore with `/system backup load name=daily-backup`.
- For portable text config use `/export` instead.

Source: MikroTik RouterOS Documentation — https://wiki.mikrotik.com
