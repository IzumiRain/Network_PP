---
id = "std/osi-model"
kind = "standard"
title = "OSI Model (ISO/IEC 7498-1)"
category = "reference/networking/fundamentals"
tags = ["osi", "fundamentals", "layers"]
version = "ISO/IEC 7498-1"
---
The OSI model divides network communication into seven layers. Each layer
serves the layer above it and consumes the layer below it.

| # | Layer | What it defines | Examples |
|---|---|---|---|
| 7 | Application | Semantics of the data for the end user | HTTP, DNS, SMTP, SSH |
| 6 | Presentation | Encoding, encryption, compression | TLS record format, JPEG, UTF-8 |
| 5 | Session | Dialog control, sessions | NetBIOS, RPC session layer |
| 4 | Transport | End-to-end delivery, ports, reliability | TCP, UDP, SCTP |
| 3 | Network | Logical addressing and routing | IPv4, IPv6, ICMP, OSPF |
| 2 | Data Link | Framing, MAC addressing, local delivery | Ethernet, ARP, VLAN, Wi-Fi |
| 1 | Physical | Bits on a medium | Copper, fiber, radio |

## How to use it in practice

- **Troubleshooting bottom-up:** check cable/link (L1), then MAC/ARP/VLAN (L2),
  then IP/gateway/routes (L3), then ports/sockets (L4), then the application
  itself (L7). Most "the app is broken" tickets die at L3.
- **Naming packets:** L2 = frame, L3 = packet, L4 = segment (TCP) or datagram
  (UDP), L5–7 = message or payload.
- **The packet visualizer in this app** maps Ethernet to L2, IPv4/IPv6 to L3,
  TCP/UDP/ICMP to L4 and HTTP to L7.

## Mnemonic (bottom-up)

"Please Do Not Throw Sausage Pizza Away" — Physical, Data Link, Network,
Transport, Session, Presentation, Application.
