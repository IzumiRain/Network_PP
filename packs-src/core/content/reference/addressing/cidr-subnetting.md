---
id = "proto/cidr-subnetting"
kind = "protocol"
title = "CIDR & Subnetting"
category = "reference/networking/addressing"
tags = ["cidr", "subnetting", "fundamentals"]
risk = "info"
version = "RFC 4632"
---
CIDR notation `a.b.c.d/p` says: the first `p` bits are the network, the rest
are hosts. The netmask is just the prefix drawn as 1s: `/24` =
`255.255.255.0`.

## The three numbers that matter

For `192.168.1.130/26`:

- **Block size** = 2^(32−26) = 64 addresses
- **Network** = the block start containing the IP = `192.168.1.128`
- **Broadcast** = block end = `192.168.1.191`, usable hosts `.129–.190`

## Common sizes

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

## Rules of thumb

- Subnets that can talk must sit inside one prefix or be routed.
- Splitting a /24 into /26s yields 4 subnets; each loses network+broadcast.
- A wildcard mask (ACL style) is the inverse of the netmask.
- Use the CIDR calculator in this app — it also splits subnets and converts
  ranges back to minimal CIDR lists.
