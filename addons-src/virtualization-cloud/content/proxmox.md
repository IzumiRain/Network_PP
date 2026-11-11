---
id = "doc/vendors/proxmox/proxmox"
kind = "vendor"
title = "Proxmox VE"
category = "reference/vendors/proxmox"
tags = ["proxmox", "pve", "cli", "lxc", "lifecycle", "kvm", "model-doc"]
risk = "high"
version = "Proxmox VE 8.x"
---
The complete Proxmox VE document: every topic for this model in one
file (v2 document model — see docs/PACK-SPEC.md, "Document model (v2 draft)").
Content is reorganised, unchanged, from this pack's original per-command
files; each topic is a section carrying its own tags via a section-tags
marker. Source attributions are kept per topic.


## Commands
<!-- section-tags: proxmox, pve, cli, lxc, lifecycle, kvm -->


### pct start / shutdown / stop

Starts, gracefully stops or hard-stops an LXC container.

#### Command

```bash
pct start 200
pct shutdown 200
pct stop 200
```

#### Notes

- `shutdown` signals init inside the container; `stop` kills it immediately.
- A stuck container that ignores shutdown usually has a hung process in its namespace.
- Confirm the VMID before running - there is no undo for `stop` beyond the container's own filesystem.

Source: Proxmox VE Administration Guide — https://pve.proxmox.com


### pct list (container inventory)

Lists all LXC containers with VMID, status and hostname.

#### Command

```bash
pct list
pct config 200
```

#### Notes

- `pct config <vmid>` shows container resources: rootfs, network interfaces, mounts.
- Container VMIDs share the same namespace as VMs - never reuse an ID.
- Enter a container shell with `pct enter <vmid>` when debugging from the host.

Source: Proxmox VE Administration Guide — https://pve.proxmox.com


### qm list (VM inventory)

Lists all KVM virtual machines with VMID, name, status and memory.

#### Command

```bash
qm list
qm status 101
qm config 101
```

#### Notes

- `qm status <vmid>` adds per-VM detail (ha state, uptime, pid).
- `qm config <vmid>` shows the VM definition: disks, NICs, boot order.
- Note the VMID - it is the key for all other qm commands.

Source: Proxmox VE Administration Guide — https://pve.proxmox.com


### qm start / shutdown / stop / reboot

Starts, gracefully stops, hard-stops or reboots a KVM virtual machine.

#### Command

```bash
qm start 101
qm shutdown 101
qm stop 101
qm reboot 101
```

#### Notes

- `shutdown` sends ACPI and waits for the guest OS; `stop` is an immediate power-off (data-loss risk).
- Always try `shutdown` first; use `stop` only when the guest is hung.
- Add `--timeout <seconds>` to shutdown to control how long it waits before giving up.
- These affect production workloads - confirm the VMID twice.

Source: Proxmox VE Administration Guide — https://pve.proxmox.com

