---
id = "cmd/paloalto/show-high-availability-state"
kind = "command"
title = "show high-availability state"
category = "reference/firewalls/palo-alto"
tags = ["palo-alto", "pan-os", "cli", "ha"]
risk = "info"
version = "PAN-OS 10.x/11.x"
---
Reports HA mode and the local/peer state, including configuration and link sync status.

## Command

```text
show high-availability state
```

## Notes

- Shows local state (active/passive/suspended) and the peer's state in one view.
- First check after a failover or when you suspect split-brain.
- Config and session sync problems appear as out-of-sync warnings here.
- Peer unreachable or version mismatch shows up before any failover happens.

Source: Palo Alto PAN-OS Administrator's Guide — https://docs.paloaltonetworks.com
