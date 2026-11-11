---
id = "doc/networking/transport/transport"
kind = "protocol"
title = "Networking: Transport"
category = "reference/networking/transport"
tags = ["tcp", "handshake", "flags", "udp", "transport", "model-doc"]
risk = "info"
---
The complete Networking: Transport document: every topic for this area in one
file (v2 document model — see docs/PACK-SPEC.md, "Document model (v2 draft)").
Content is reorganised, unchanged, from this pack's original per-topic
files; each topic is a section carrying its own tags via a section-tags
marker.

## TCP Handshake & Flags
<!-- section-tags: tcp, handshake, flags -->

Version: RFC 9293

TCP is connection-oriented: before data flows, endpoints agree on sequence
numbers with a three-way handshake.

#### Three-way handshake

```
client                     server
  | ---- SYN (seq=x) ----->  |   "I want to talk, starting at x"
  | <--- SYN+ACK (seq=y,     |   "OK, I start at y, got your x"
  |       ack=x+1) ---->     |
  | ---- ACK (ack=y+1) --->  |   connection ESTABLISHED
```

Closing: FIN → FIN+ACK → ACK (each direction independently). A RST kills the
connection immediately — that's what a refused port returns.

#### Flags you will see in scans and captures

| Flag | Meaning | In the field |
|---|---|---|
| SYN | start connection | scanner's probe |
| SYN+ACK | port open on server | what an open port answers |
| RST | abort | closed port / blocked |
| ACK | acknowledge data | mid-connection traffic |
| PSH | push to app now | carries payload |
| FIN | graceful close | normal end |
| URG/ECE/CWR | urgent/ECN | rare |

#### What a scanner infers

- SYN → **SYN+ACK**: port open
- SYN → **RST**: port closed
- SYN → silence: filtered (firewall drop) — this is the three-state logic the
  Target Scan tool reports.


## UDP — User Datagram Protocol
<!-- section-tags: udp, transport -->

Version: RFC 768

UDP is connectionless: an 8-byte header (source port, destination port,
length, checksum) and then data. No handshake, no retransmission, no ordering.

#### When UDP wins

- Latency beats reliability: voice, video, games
- One request, one answer: DNS (small queries), DHCP, NTP, SNMP
- Broadcast/multicast: mDNS, discovery protocols

#### Diagnostics gotchas

- An open UDP port may not answer anything (that's normal — DNS only answers
  when asked). "No reply" is not "closed".
- ICMP "port unreachable" is how a closed UDP port is usually reported —
  tools map that back to closed; silence means open-or-filtered.
- The header checksum is optional in IPv4 (0 = not computed) and mandatory
  in IPv6.

Build a UDP datagram in Tools → Packet Lab (parse mode) and watch the 8-byte
header next to a TCP segment's 20.

