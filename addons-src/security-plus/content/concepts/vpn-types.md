---
id = "std/security-plus/vpn-types"
kind = "protocol"
title = "VPN types summary"
category = "reference/security/security-plus/concepts"
tags = ["security-plus", "vpn", "ipsec", "tls", "split-tunnel"]
version = "SY0-701"
---
VPNs encrypt traffic across untrusted networks; the type is defined by the endpoints and the tunnel scope.

## Topologies

```text
site-to-site:   LAN A <-> gateway <-> Internet <-> gateway <-> LAN B   (IPsec tunnel mode)
remote-access:  client -> gateway -> private network                   (IPsec/IKEv2, TLS/SSL VPN, WireGuard)
```

## Notes

- Full tunnel sends ALL client traffic through the VPN; split tunnel sends only corporate-bound traffic and leaves the rest on the local link (risk: an infected client bridges the two worlds).
- TLS/SSL VPN traverses firewalls easily (TCP 443) and can run in a browser; IPsec operates at the network layer (tunnel vs transport mode).
- Always-on VPN with a kill switch is the secure-by-default posture for remote workers.

Source: CompTIA Security+ SY0-701 Exam Objectives — https://www.comptia.org
