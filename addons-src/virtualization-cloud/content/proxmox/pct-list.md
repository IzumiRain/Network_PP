---
id = "cmd/proxmox/pct-list"
kind = "command"
title = "pct list (container inventory)"
category = "reference/vendors/proxmox"
tags = ["proxmox", "pve", "cli", "lxc"]
risk = "info"
version = "Proxmox VE 8.x"
---
Lists all LXC containers with VMID, status and hostname.

## Command

```bash
pct list
pct config 200
```

## Notes

- `pct config <vmid>` shows container resources: rootfs, network interfaces, mounts.
- Container VMIDs share the same namespace as VMs - never reuse an ID.
- Enter a container shell with `pct enter <vmid>` when debugging from the host.

Source: Proxmox VE Administration Guide — https://pve.proxmox.com
