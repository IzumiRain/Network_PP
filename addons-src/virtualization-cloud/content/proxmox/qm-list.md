---
id = "cmd/proxmox/qm-list"
kind = "command"
title = "qm list (VM inventory)"
category = "reference/virtualization/proxmox"
tags = ["proxmox", "pve", "cli", "kvm"]
risk = "info"
version = "Proxmox VE 8.x"
---
Lists all KVM virtual machines with VMID, name, status and memory.

## Command

```bash
qm list
qm status 101
qm config 101
```

## Notes

- `qm status <vmid>` adds per-VM detail (ha state, uptime, pid).
- `qm config <vmid>` shows the VM definition: disks, NICs, boot order.
- Note the VMID - it is the key for all other qm commands.

Source: Proxmox VE Administration Guide — https://pve.proxmox.com
