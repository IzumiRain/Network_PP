---
id = "proto/dns-basics"
kind = "protocol"
title = "DNS — Domain Name System"
category = "reference/networking/services"
tags = ["dns", "services"]
risk = "info"
version = "RFC 1034/1035"
---
DNS maps names to data (mostly IP addresses). Port 53, over UDP for queries
(TCP for large answers/zone transfers).

## Record types

| Record | Meaning |
|---|---|
| A / AAAA | name → IPv4 / IPv6 |
| CNAME | alias to another name |
| MX | mail servers (with priority) |
| NS | authoritative name servers |
| TXT | arbitrary text (SPF, verification) |
| SOA | zone metadata (serial, refresh) |
| PTR | reverse: IP → name (in-addr.arpa) |
| SRV | service discovery (port + host) |

## Resolution path

Client cache → configured resolver (recursive) → root → TLD → authoritative.
Negative answers are cached too (NXDOMAIN has a TTL).

## Field diagnostics

- `dig +short example.com` — fast answer
- `dig @1.1.1.1 example.com MX` — ask a specific server
- Wrong-but-consistent answer → local/static entry or split-horizon
- Works by IP, fails by name → DNS; check `resolv.conf` / adapter DNS first
