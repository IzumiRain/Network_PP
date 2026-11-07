---
id = "cmd/proxmox/qm-vm-lifecycle"
kind = "command"
title = "qm start / shutdown / stop / reboot"
category = "reference/vendors/proxmox"
tags = ["proxmox", "pve", "cli", "kvm", "lifecycle"]
risk = "high"
version = "Proxmox VE 8.x"
---
Starts, gracefully stops, hard-stops or reboots a KVM virtual machine.

## Command

```bash
qm start 101
qm shutdown 101
qm stop 101
qm reboot 101
```

## Notes

- `shutdown` sends ACPI and waits for the guest OS; `stop` is an immediate power-off (data-loss risk).
- Always try `shutdown` first; use `stop` only when the guest is hung.
- Add `--timeout <seconds>` to shutdown to control how long it waits before giving up.
- These affect production workloads - confirm the VMID twice.

Source: Proxmox VE Administration Guide — https://pve.proxmox.com
