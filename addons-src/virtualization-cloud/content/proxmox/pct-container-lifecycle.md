---
id = "cmd/proxmox/pct-container-lifecycle"
kind = "command"
title = "pct start / shutdown / stop"
category = "reference/virtualization/proxmox"
tags = ["proxmox", "pve", "cli", "lxc", "lifecycle"]
risk = "high"
version = "Proxmox VE 8.x"
---
Starts, gracefully stops or hard-stops an LXC container.

## Command

```bash
pct start 200
pct shutdown 200
pct stop 200
```

## Notes

- `shutdown` signals init inside the container; `stop` kills it immediately.
- A stuck container that ignores shutdown usually has a hung process in its namespace.
- Confirm the VMID before running - there is no undo for `stop` beyond the container's own filesystem.

Source: Proxmox VE Administration Guide — https://pve.proxmox.com
