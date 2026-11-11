---
id = "doc/networking/services/services"
kind = "protocol"
title = "Networking: Services"
category = "reference/networking/services"
tags = ["dhcp", "services", "dns", "model-doc"]
risk = "info"
---
The complete Networking: Services document: every topic for this area in one
file (v2 document model — see docs/PACK-SPEC.md, "Document model (v2 draft)").
Content is reorganised, unchanged, from this pack's original per-topic
files; each topic is a section carrying its own tags via a section-tags
marker.

## DHCP — Dynamic Host Configuration
<!-- section-tags: dhcp, services -->

Version: RFC 2131

DHCP hands out IP configuration. UDP: client 68 → server 67.

#### DORA

1. **Discover** — client broadcasts "any DHCP server?"
2. **Offer** — server proposes an address
3. **Request** — client asks for the offer it liked
4. **Ack** — server confirms lease (IP, mask, gateway, DNS, lease time)

#### Reading the symptoms

- Address `169.254.x.x` (APIPA) → no DHCP offer arrived: server down, VLAN
  without relay, or switchport issue.
- Correct IP but no internet → check gateway and DNS, not DHCP.
- Duplicate address warnings → two DHCP servers or a static collision.
- Renewal at ~50% of lease; rebinding at ~87.5%.

On Windows `ipconfig /release` + `/renew` restarts the dance; on Linux the
client tool varies (dhclient, NetworkManager, systemd-networkd).


## DNS — Domain Name System
<!-- section-tags: dns, services -->

Version: RFC 1034/1035

DNS maps names to data (mostly IP addresses). Port 53, over UDP for queries
(TCP for large answers/zone transfers).

#### Record types

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

#### Resolution path

Client cache → configured resolver (recursive) → root → TLD → authoritative.
Negative answers are cached too (NXDOMAIN has a TTL).

#### Field diagnostics

- `dig +short example.com` — fast answer
- `dig @1.1.1.1 example.com MX` — ask a specific server
- Wrong-but-consistent answer → local/static entry or split-horizon
- Works by IP, fails by name → DNS; check `resolv.conf` / adapter DNS first

