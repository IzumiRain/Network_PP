---
id = "std/tcpip-model"
kind = "standard"
title = "TCP/IP Model"
category = "reference/networking/fundamentals"
tags = ["tcpip", "fundamentals"]
version = "RFC 1122"
---
The TCP/IP model (four layers) is what real networks implement; OSI is the
teaching vocabulary. Mapping:

| TCP/IP layer | OSI layers | Examples |
|---|---|---|
| Application | 5–7 | HTTP, DNS, SSH, TLS |
| Transport | 4 | TCP, UDP |
| Internet | 3 | IPv4, IPv6, ICMP |
| Link | 1–2 | Ethernet, ARP, Wi-Fi |

## Encapsulation

Each layer wraps the layer above: an HTTP message becomes a TCP segment
(with ports), which becomes an IP packet (with addresses), which becomes an
Ethernet frame (with MACs), which becomes bits. The receiving side unwraps in
reverse. See the Packet visualizer in Tools → Packet Lab to watch this happen
byte by byte.

## Why two models

- OSI gives you precise words for a problem ("that's a layer 2 issue").
- TCP/IP reflects the protocols that actually ship.
- In the field people say "layer 2/3/7" (OSI numbers) while running the
  TCP/IP stack.
