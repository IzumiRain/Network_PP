---
id = "proto/dhcp-basics"
kind = "protocol"
title = "DHCP — Dynamic Host Configuration"
category = "reference/networking/services"
tags = ["dhcp", "services"]
risk = "info"
version = "RFC 2131"
---
DHCP hands out IP configuration. UDP: client 68 → server 67.

## DORA

1. **Discover** — client broadcasts "any DHCP server?"
2. **Offer** — server proposes an address
3. **Request** — client asks for the offer it liked
4. **Ack** — server confirms lease (IP, mask, gateway, DNS, lease time)

## Reading the symptoms

- Address `169.254.x.x` (APIPA) → no DHCP offer arrived: server down, VLAN
  without relay, or switchport issue.
- Correct IP but no internet → check gateway and DNS, not DHCP.
- Duplicate address warnings → two DHCP servers or a static collision.
- Renewal at ~50% of lease; rebinding at ~87.5%.

On Windows `ipconfig /release` + `/renew` restarts the dance; on Linux the
client tool varies (dhclient, NetworkManager, systemd-networkd).
