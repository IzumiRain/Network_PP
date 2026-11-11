---
id = "doc/networking/fundamentals/fundamentals"
kind = "standard"
title = "Networking: Fundamentals"
category = "reference/networking/fundamentals"
tags = ["osi", "fundamentals", "layers", "tcpip", "model-doc"]
---
The complete Networking: Fundamentals document: every topic for this area in one
file (v2 document model — see docs/PACK-SPEC.md, "Document model (v2 draft)").
Content is reorganised, unchanged, from this pack's original per-topic
files; each topic is a section carrying its own tags via a section-tags
marker.

## OSI Model (ISO/IEC 7498-1)
<!-- section-tags: osi, fundamentals, layers -->

Version: ISO/IEC 7498-1

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

#### How to use it in practice

- **Troubleshooting bottom-up:** check cable/link (L1), then MAC/ARP/VLAN (L2),
  then IP/gateway/routes (L3), then ports/sockets (L4), then the application
  itself (L7). Most "the app is broken" tickets die at L3.
- **Naming packets:** L2 = frame, L3 = packet, L4 = segment (TCP) or datagram
  (UDP), L5–7 = message or payload.
- **The packet visualizer in this app** maps Ethernet to L2, IPv4/IPv6 to L3,
  TCP/UDP/ICMP to L4 and HTTP to L7.

#### Mnemonic (bottom-up)

"Please Do Not Throw Sausage Pizza Away" — Physical, Data Link, Network,
Transport, Session, Presentation, Application.


## TCP/IP Model
<!-- section-tags: tcpip, fundamentals -->

Version: RFC 1122

The TCP/IP model (four layers) is what real networks implement; OSI is the
teaching vocabulary. Mapping:

| TCP/IP layer | OSI layers | Examples |
|---|---|---|
| Application | 5–7 | HTTP, DNS, SSH, TLS |
| Transport | 4 | TCP, UDP |
| Internet | 3 | IPv4, IPv6, ICMP |
| Link | 1–2 | Ethernet, ARP, Wi-Fi |

#### Encapsulation

Each layer wraps the layer above: an HTTP message becomes a TCP segment
(with ports), which becomes an IP packet (with addresses), which becomes an
Ethernet frame (with MACs), which becomes bits. The receiving side unwraps in
reverse. See the Packet visualizer in Tools → Packet Lab to watch this happen
byte by byte.

#### Why two models

- OSI gives you precise words for a problem ("that's a layer 2 issue").
- TCP/IP reflects the protocols that actually ship.
- In the field people say "layer 2/3/7" (OSI numbers) while running the
  TCP/IP stack.

