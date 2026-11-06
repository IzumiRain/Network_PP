---
id = "proto/ipv4-basics"
kind = "protocol"
title = "IPv4 Addressing"
category = "reference/networking/addressing"
tags = ["ipv4", "addressing"]
risk = "info"
version = "RFC 791"
---
An IPv4 address is a 32-bit number written as four octets (e.g. `192.168.1.10`).
The address has two parts: **network prefix** (which network) and **host**
(which device on it) — the split point is the prefix length (CIDR) or netmask.

## Structure of an octet

Each octet is 0–255. In binary, `192.168.1.10` is
`11000000.10101000.00000001.00001010` — the calculators in this app show this
live.

## Special ranges

| Range | Purpose |
|---|---|
| `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16` | Private (RFC 1918) |
| `127.0.0.0/8` | Loopback |
| `169.254.0.0/16` | Link-local / APIPA (DHCP failed) |
| `224.0.0.0/4` | Multicast |
| `240.0.0.0/4` | Reserved (incl. `255.255.255.255` broadcast) |
| `0.0.0.0/8` | "This network" (0.0.0.0 = unset/any) |

## Header essentials (see Packet Lab → Build)

Version/IHL, DSCP/ECN, total length, ID + flags + fragment offset, TTL,
protocol (6=TCP, 17=UDP, 1=ICMP), header checksum, source, destination.
TTL decreasing by one per router hop is what makes traceroute work.
