---
id = "cmd/fortigate/execute-update-now"
kind = "command"
title = "execute update-now"
category = "reference/vendors/fortinet/fortigate/system"
tags = ["fortigate", "fortios", "cli", "updates"]
risk = "medium"
version = "FortiOS 7.4"
---
Triggers an immediate FortiGuard update pull (AV, IPS, web filter databases) instead of waiting for the scheduled update.

## Command

```cisco
execute update-now
```

## Notes

- Downloads and installs new signature versions, so it changes protection DB state on the unit.
- Causes brief disk and CPU activity; avoid running it during traffic peaks on small models.
- Verify the result afterwards in the FortiGuard dashboard or the event log.

Source: Fortinet FortiOS Administration Guide — https://docs.fortinet.com
