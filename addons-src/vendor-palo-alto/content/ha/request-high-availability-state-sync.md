---
id = "cmd/paloalto/request-high-availability-state-sync"
kind = "command"
title = "request high-availability state sync"
category = "reference/vendors/palo-alto"
tags = ["palo-alto", "pan-os", "cli", "ha"]
risk = "medium"
version = "PAN-OS 10.x/11.x"
---
Triggers synchronization of configuration/state between the HA peers.

## Command

```text
request high-availability state sync
```

## Notes

- Use after resolving configuration drift between active and passive members.
- Can briefly impact the peer while it syncs; run it inside a maintenance window.
- Verify completion with `show high-availability state` (sync counters back to normal).
- Do not run concurrently with a commit or software upgrade.

Source: Palo Alto PAN-OS Administrator's Guide — https://docs.paloaltonetworks.com
