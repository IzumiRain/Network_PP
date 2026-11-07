---
id = "cmd/juniper/show-lldp-neighbors"
kind = "command"
title = "show lldp neighbors"
category = "reference/devices/juniper"
tags = ["juniper", "junos", "cli", "lldp", "topology"]
risk = "info"
version = "Junos OS 22.x"
---
Lists LLDP-discovered neighbors per local interface with system name and port ID.

## Command

```text
show lldp neighbors
show lldp neighbors interface ge-0/0/1
```

## Notes

- Handy for building patch maps and verifying cabling between switches and routers.
- Shows chassis ID, port description and system name advertised by the peer.
- Per-interface form narrows output on large leaf/spine fabrics.
- Requires LLDP enabled (default on most modern Junos releases).

Source: Juniper Junos CLI Reference — https://www.juniper.net/documentation
