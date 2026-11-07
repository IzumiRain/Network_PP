---
id = "cmd/juniper/commit"
kind = "command"
title = "commit (configuration mode)"
category = "reference/devices/juniper"
tags = ["juniper", "junos", "cli", "config"]
risk = "medium"
version = "Junos OS 22.x"
---
Configuration-mode command that validates and activates all staged configuration changes.

## Command

```text
commit
commit confirmed 5
commit comment "change ticket 1234"
```

## Notes

- Nothing changes on the device until you commit; `commit check` validates without activating.
- `commit confirmed 5` auto-reverts after 5 minutes unless you commit again - the remote-safe way to test risky changes.
- `commit comment` logs the change reason into the commit history.
- `show system commit` lists past commits with time, user and comment.

Source: Juniper Junos CLI Reference — https://www.juniper.net/documentation
