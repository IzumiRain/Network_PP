---
id = "doc/networking/switching/switching"
kind = "protocol"
title = "Networking: Switching"
category = "reference/networking/switching"
tags = ["arp", "l2", "switching", "ethernet", "frames", "vlan", "trunking", "model-doc"]
risk = "info"
---
The complete Networking: Switching document: every topic for this area in one
file (v2 document model — see docs/PACK-SPEC.md, "Document model (v2 draft)").
Content is reorganised, unchanged, from this pack's original per-topic
files; each topic is a section carrying its own tags via a section-tags
marker.

## ARP — Address Resolution Protocol
<!-- section-tags: arp, l2, switching -->

Version: RFC 826

ARP answers "who has this IPv4 address?" on the local L2 segment and caches
the MAC answer. Ethernet frames deliver to MACs, not IPs — ARP is the glue
between L3 and L2.

#### Flow

1. Host A wants to send to 192.168.1.7 — not in cache
2. Broadcast request: "Who has 192.168.1.7? tell 192.168.1.10"
3. Host B (owning the IP) unicasts its MAC
4. A caches the pair (entry ages out — minutes)

#### Using the tables

- `ip neigh` (Linux) / `arp -a` (Windows/macOS): IP ↔ MAC ↔ state
- **REACHABLE** = recently confirmed; **STALE** = old, will re-verify;
  **INCOMPLETE** = no answer — host likely down
- The gateway entry must exist or you have no L3 exit at all
- Local Devices in this app reads these tables and maps vendor from the MAC
  OUI prefix

#### Classic traps

- Two IPs sharing one MAC in the table = conflict or virtual IP (HA cluster)
- MAC that matches no known vendor → spoofing or exotic VM
- ARP works only inside the subnet — cross-subnet is routed, not ARP'd


## Ethernet Frame (IEEE 802.3)
<!-- section-tags: ethernet, l2, frames -->

Version: IEEE 802.3

The L2 frame layout — exactly what the Packet visualizer shows first:

```
| Destination MAC | Source MAC | EtherType | Payload          | FCS |
| 6 bytes         | 6 bytes    | 2 bytes   | 46–1500 bytes    | 4 B |
```

#### Fields worth knowing

- **MAC** — 48 bits; first half is the OUI (vendor), second half device
  serial. `FF:FF:FF:FF:FF:FF` = broadcast, `01:00:5E:…` = IPv4 multicast.
- **EtherType** — what rides inside: `0x0800` IPv4, `0x86DD` IPv6,
  `0x0806` ARP, `0x8100` 802.1Q VLAN tag (tag sits before the real EtherType).
- **FCS** — CRC check; bad FCS frames are dropped (duplex mismatch legacy).

#### MTU

Standard payload max is 1500 bytes. Jumbo frames (usually 9000) must match
end-to-end including switches. Ping tests that assume size must account for
28 bytes of IP+ICMP header: `ping -l 1472` (Windows) tests a 1500 MTU path.


## VLANs & Trunking (802.1Q)
<!-- section-tags: vlan, l2, trunking -->

Version: IEEE 802.1Q

A VLAN splits one physical switch into many isolated L2 networks. Each frame
on a trunk carries a 4-byte 802.1Q tag with a 12-bit VLAN ID (1–4094).

#### Port roles

- **Access port** — one untagged VLAN (PCs, printers, APs)
- **Trunk port** — carries many tagged VLANs (switch↔switch, switch↔router,
  hypervisor)
- **Native VLAN** — the one untagged VLAN allowed on a trunk (keep it unused
  and identical on both ends)

#### Troubleshooting checklist

1. Same VLAN on both ends of the path? (show vlan / show interfaces trunk)
2. VLAN allowed on the trunk, not just created?
3. Native VLAN mismatch warnings — fix, don't ignore
4. Inter-VLAN traffic needs a router/L3 switch — VLANs don't route by themselves
5. DHCP on VLAN 10 with server on VLAN 1 → needs `ip helper-address` relay

Symptom pattern: "some devices talk, others don't, IP settings look right"
is almost always an L2 VLAN/native VLAN problem, not IP.

