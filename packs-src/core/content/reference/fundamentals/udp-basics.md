---
id = "proto/udp-basics"
kind = "protocol"
title = "UDP — User Datagram Protocol"
category = "reference/networking/transport"
tags = ["udp", "transport"]
risk = "info"
version = "RFC 768"
---
UDP is connectionless: an 8-byte header (source port, destination port,
length, checksum) and then data. No handshake, no retransmission, no ordering.

## When UDP wins

- Latency beats reliability: voice, video, games
- One request, one answer: DNS (small queries), DHCP, NTP, SNMP
- Broadcast/multicast: mDNS, discovery protocols

## Diagnostics gotchas

- An open UDP port may not answer anything (that's normal — DNS only answers
  when asked). "No reply" is not "closed".
- ICMP "port unreachable" is how a closed UDP port is usually reported —
  tools map that back to closed; silence means open-or-filtered.
- The header checksum is optional in IPv4 (0 = not computed) and mandatory
  in IPv6.

Build a UDP datagram in Tools → Packet Lab (parse mode) and watch the 8-byte
header next to a TCP segment's 20.
