---
id = "proto/vlan-basics"
kind = "protocol"
title = "VLANs & Trunking (802.1Q)"
category = "reference/networking/switching"
tags = ["vlan", "l2", "trunking"]
risk = "info"
version = "IEEE 802.1Q"
---
A VLAN splits one physical switch into many isolated L2 networks. Each frame
on a trunk carries a 4-byte 802.1Q tag with a 12-bit VLAN ID (1–4094).

## Port roles

- **Access port** — one untagged VLAN (PCs, printers, APs)
- **Trunk port** — carries many tagged VLANs (switch↔switch, switch↔router,
  hypervisor)
- **Native VLAN** — the one untagged VLAN allowed on a trunk (keep it unused
  and identical on both ends)

## Troubleshooting checklist

1. Same VLAN on both ends of the path? (show vlan / show interfaces trunk)
2. VLAN allowed on the trunk, not just created?
3. Native VLAN mismatch warnings — fix, don't ignore
4. Inter-VLAN traffic needs a router/L3 switch — VLANs don't route by themselves
5. DHCP on VLAN 10 with server on VLAN 1 → needs `ip helper-address` relay

Symptom pattern: "some devices talk, others don't, IP settings look right"
is almost always an L2 VLAN/native VLAN problem, not IP.
