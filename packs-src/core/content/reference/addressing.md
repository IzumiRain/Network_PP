---
id = "doc/networking/addressing/addressing"
kind = "protocol"
title = "Networking: Addressing"
category = "reference/networking/addressing"
tags = ["cidr", "subnetting", "fundamentals", "ipv4", "addressing", "model-doc"]
risk = "info"
---
The complete Networking: Addressing document: every topic for this area in one
file (v2 document model — see docs/PACK-SPEC.md, "Document model (v2 draft)").
Content is reorganised, unchanged, from this pack's original per-topic
files; each topic is a section carrying its own tags via a section-tags
marker.

## CIDR & Subnetting
<!-- section-tags: cidr, subnetting, fundamentals -->

Version: RFC 4632

CIDR notation `a.b.c.d/p` says: the first `p` bits are the network, the rest
are hosts. The netmask is just the prefix drawn as 1s: `/24` =
`255.255.255.0`.

#### The three numbers that matter

For `192.168.1.130/26`:

- **Block size** = 2^(32−26) = 64 addresses
- **Network** = the block start containing the IP = `192.168.1.128`
- **Broadcast** = block end = `192.168.1.191`, usable hosts `.129–.190`

#### Common sizes

| CIDR | Mask | Addresses | Typical use |
|---|---|---|---|
| /8 | 255.0.0.0 | 16,777,216 | huge ISP / legacy |
| /16 | 255.255.0.0 | 65,536 | campus |
| /24 | 255.255.255.0 | 256 | standard LAN |
| /26 | 255.255.255.192 | 64 | small segment |
| /29 | 255.255.255.248 | 8 | point-to-point + spares |
| /30 | 255.255.255.252 | 4 | point-to-point |
| /31 | 255.255.255.254 | 2 | point-to-point (RFC 3021) |
| /32 | 255.255.255.255 | 1 | single host route |

#### Rules of thumb

- Subnets that can talk must sit inside one prefix or be routed.
- Splitting a /24 into /26s yields 4 subnets; each loses network+broadcast.
- A wildcard mask (ACL style) is the inverse of the netmask.
- Use the CIDR calculator in this app — it also splits subnets and converts
  ranges back to minimal CIDR lists.


## IPv4 Addressing
<!-- section-tags: ipv4, addressing -->

Version: RFC 791

An IPv4 address is a 32-bit number written as four octets (e.g. `192.168.1.10`).
The address has two parts: **network prefix** (which network) and **host**
(which device on it) — the split point is the prefix length (CIDR) or netmask.

#### Structure of an octet

Each octet is 0–255. In binary, `192.168.1.10` is
`11000000.10101000.00000001.00001010` — the calculators in this app show this
live.

#### Special ranges

| Range | Purpose |
|---|---|
| `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16` | Private (RFC 1918) |
| `127.0.0.0/8` | Loopback |
| `169.254.0.0/16` | Link-local / APIPA (DHCP failed) |
| `224.0.0.0/4` | Multicast |
| `240.0.0.0/4` | Reserved (incl. `255.255.255.255` broadcast) |
| `0.0.0.0/8` | "This network" (0.0.0.0 = unset/any) |

#### Header essentials (see Packet Lab → Build)

Version/IHL, DSCP/ECN, total length, ID + flags + fragment offset, TTL,
protocol (6=TCP, 17=UDP, 1=ICMP), header checksum, source, destination.
TTL decreasing by one per router hop is what makes traceroute work.

