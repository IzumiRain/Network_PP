---
id = "cmd/f5-bigip/show-cm-sync-status"
kind = "command"
title = "show cm sync-status"
category = "reference/vendors/f5/bigip/ha"
tags = ["f5", "bigip", "tmsh", "ha"]
risk = "info"
version = "BIG-IP 16.1"
---
Shows the device group configuration sync status: whether both HA peers hold the same configuration.

## Command

```bash
tmsh show cm sync-status
```

## Notes

- In Sync / green is the healthy state; Changes Pending means one side has unsynced modifications.
- Fix sync issues before failover testing — an out-of-sync standby restores stale config when it becomes active.
- Details include the device group name and the last sync direction.

Source: F5 BIG-IP tmsh Reference Guide — https://techdocs.f5.com
