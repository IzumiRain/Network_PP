---
id = "cmd/juniper/show-configuration"
kind = "command"
title = "show configuration"
category = "reference/vendors/juniper"
tags = ["juniper", "junos", "cli", "config"]
risk = "info"
version = "Junos OS 22.x"
---
Operational-mode command that prints the candidate configuration in curly-brace format.

## Command

```text
show configuration
show configuration interfaces ge-0/0/0
show configuration | display set
```

## Notes

- Junos applies config via a candidate model: what you see here becomes active only after `commit`.
- Narrow to a subtree by path, or use `| display set` to get copy-pasteable `set` commands.
- Diff candidate against a past commit: `show configuration | compare rollback 1`.
- Large configs output faster when scoped to one hierarchy level.

Source: Juniper Junos CLI Reference — https://www.juniper.net/documentation
