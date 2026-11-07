---
id = "cmd/asa/access-list-config"
kind = "command"
title = "access-list / access-group (interface ACL)"
category = "reference/firewalls/cisco-asa/acl"
tags = ["asa", "cisco", "cli", "acl", "config"]
risk = "medium"
version = "ASA 9.16"
---
Creates an extended ACL and binds it to an interface direction with access-group.

## Command

```cisco
access-list INSIDE_IN extended permit tcp 10.1.1.0 255.255.255.0 any eq 443
access-list INSIDE_IN extended deny ip any any log
access-group INSIDE_IN in interface inside
```

## Notes

- The first `access-group ... in interface <name>` on a formerly unfiltered interface immediately denies everything not permitted - it is service-affecting.
- Entries cannot be reordered easily; new ACEs append at the end unless inserted by line number (`line <n>`).
- Use `no access-list INSIDE_IN line 2` to remove a single entry; `no access-list <name>` deletes the whole ACL.
- An empty or missing ACL on an interface falls back to security-level defaults.

Source: Cisco ASA Series CLI Configuration Guide — https://www.cisco.com
