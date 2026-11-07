---
id = "cmd/juniper/rollback"
kind = "command"
title = "rollback (configuration mode)"
category = "reference/devices/juniper"
tags = ["juniper", "junos", "cli", "config"]
risk = "medium"
version = "Junos OS 22.x"
---
Configuration-mode command that loads a previously committed configuration into the candidate.

## Command

```text
rollback 1
commit
```

## Notes

- `rollback 0` loads the currently committed config (discarding uncommitted edits); `rollback 1` the one before it.
- Takes effect only after a following `commit` - the device stays unchanged until then.
- `show system commit` shows which rollback number maps to which change.
- For remote work, pair with `commit confirmed` so a bad change reverts automatically.

Source: Juniper Junos CLI Reference — https://www.juniper.net/documentation
