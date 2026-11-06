---
id = "proto/arp"
kind = "protocol"
title = "ARP — Address Resolution Protocol"
category = "reference/networking/switching"
tags = ["arp", "l2", "switching"]
risk = "info"
version = "RFC 826"
---
ARP answers "who has this IPv4 address?" on the local L2 segment and caches
the MAC answer. Ethernet frames deliver to MACs, not IPs — ARP is the glue
between L3 and L2.

## Flow

1. Host A wants to send to 192.168.1.7 — not in cache
2. Broadcast request: "Who has 192.168.1.7? tell 192.168.1.10"
3. Host B (owning the IP) unicasts its MAC
4. A caches the pair (entry ages out — minutes)

## Using the tables

- `ip neigh` (Linux) / `arp -a` (Windows/macOS): IP ↔ MAC ↔ state
- **REACHABLE** = recently confirmed; **STALE** = old, will re-verify;
  **INCOMPLETE** = no answer — host likely down
- The gateway entry must exist or you have no L3 exit at all
- Local Devices in this app reads these tables and maps vendor from the MAC
  OUI prefix

## Classic traps

- Two IPs sharing one MAC in the table = conflict or virtual IP (HA cluster)
- MAC that matches no known vendor → spoofing or exotic VM
- ARP works only inside the subnet — cross-subnet is routed, not ARP'd
