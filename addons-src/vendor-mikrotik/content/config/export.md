---
id = "cmd/mikrotik/export"
kind = "command"
title = "/export (configuration export)"
category = "reference/devices/mikrotik"
tags = ["mikrotik", "routeros", "cli", "config"]
risk = "info"
version = "RouterOS 7.x"
---
Prints the whole RouterOS configuration as importable .rsc commands.

## Command

```text
/export
/export file=config-export
/export hide-sensitive
```

## Notes

- In RouterOS 7 the plain `export` command works from the top-level menu and inside any submenu.
- With `file=` the output goes to a .rsc file on flash; retrieve it via the Files menu, SFTP or FTP.
- `hide-sensitive` omits passwords and keys - use it when sharing configs for support.
- Apply on another unit with `/import file=config-export.rsc`.

Source: MikroTik RouterOS Documentation — https://wiki.mikrotik.com
