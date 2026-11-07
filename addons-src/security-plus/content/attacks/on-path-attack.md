---
id = "std/security-plus/on-path-attack"
kind = "error"
title = "On-path attack"
category = "reference/security/security-plus/attacks"
tags = ["security-plus", "on-path", "mitm", "evil-twin", "arp-spoofing"]
version = "SY0-701"
---
An on-path attack (formerly man-in-the-middle) places the attacker between two endpoints so traffic can be read or altered in transit.

## How it happens

```text
client -> [attacker relays, reads, may modify] -> server
        via ARP spoofing, rogue Wi-Fi AP (evil twin), DNS poisoning, TLS stripping
```

## Notes

- Untrusted Wi-Fi hotspots are classic setup ground: the attacker ARP-spoofs the gateway or runs an evil twin AP.
- Plaintext protocols (HTTP, DNS, POP3) are fully exposed; TLS sessions get proxied with self-signed certs users click through.
- Defenses: HTTPS/TLS everywhere with certificate validation, HSTS, 802.1X, dynamic ARP inspection, DHCP snooping, VPN on untrusted networks.

Source: CompTIA Security+ SY0-701 Exam Objectives — https://www.comptia.org
