---
id = "proto/ethernet-frame"
kind = "protocol"
title = "Ethernet Frame (IEEE 802.3)"
category = "reference/networking/switching"
tags = ["ethernet", "l2", "frames"]
risk = "info"
version = "IEEE 802.3"
---
The L2 frame layout — exactly what the Packet visualizer shows first:

```
| Destination MAC | Source MAC | EtherType | Payload          | FCS |
| 6 bytes         | 6 bytes    | 2 bytes   | 46–1500 bytes    | 4 B |
```

## Fields worth knowing

- **MAC** — 48 bits; first half is the OUI (vendor), second half device
  serial. `FF:FF:FF:FF:FF:FF` = broadcast, `01:00:5E:…` = IPv4 multicast.
- **EtherType** — what rides inside: `0x0800` IPv4, `0x86DD` IPv6,
  `0x0806` ARP, `0x8100` 802.1Q VLAN tag (tag sits before the real EtherType).
- **FCS** — CRC check; bad FCS frames are dropped (duplex mismatch legacy).

## MTU

Standard payload max is 1500 bytes. Jumbo frames (usually 9000) must match
end-to-end including switches. Ping tests that assume size must account for
28 bytes of IP+ICMP header: `ping -l 1472` (Windows) tests a 1500 MTU path.
