---
id = "proto/tcp-handshake"
kind = "protocol"
title = "TCP Handshake & Flags"
category = "reference/networking/transport"
tags = ["tcp", "handshake", "flags"]
risk = "info"
version = "RFC 9293"
---
TCP is connection-oriented: before data flows, endpoints agree on sequence
numbers with a three-way handshake.

## Three-way handshake

```
client                     server
  | ---- SYN (seq=x) ----->  |   "I want to talk, starting at x"
  | <--- SYN+ACK (seq=y,     |   "OK, I start at y, got your x"
  |       ack=x+1) ---->     |
  | ---- ACK (ack=y+1) --->  |   connection ESTABLISHED
```

Closing: FIN → FIN+ACK → ACK (each direction independently). A RST kills the
connection immediately — that's what a refused port returns.

## Flags you will see in scans and captures

| Flag | Meaning | In the field |
|---|---|---|
| SYN | start connection | scanner's probe |
| SYN+ACK | port open on server | what an open port answers |
| RST | abort | closed port / blocked |
| ACK | acknowledge data | mid-connection traffic |
| PSH | push to app now | carries payload |
| FIN | graceful close | normal end |
| URG/ECE/CWR | urgent/ECN | rare |

## What a scanner infers

- SYN → **SYN+ACK**: port open
- SYN → **RST**: port closed
- SYN → silence: filtered (firewall drop) — this is the three-state logic the
  Target Scan tool reports.
