---
id = "cmd/asa/packet-tracer"
kind = "command"
title = "packet-tracer (simulated path)"
category = "reference/vendors/cisco/asa/troubleshooting"
tags = ["asa", "cisco", "cli", "troubleshooting"]
risk = "info"
version = "ASA 9.16"
---
Simulates a packet through the ASA data path and reports the first drop phase - the key diagnostic for why traffic is blocked.

## Command

```cisco
packet-tracer input inside tcp 10.1.1.1 12345 8.8.8.8 443
packet-tracer input inside udp 10.1.1.1 12345 8.8.8.8 53 detailed
```

## Notes

- The output lists every phase (ACL, NAT, inspect, crypto, flow creation); the phase marked (drop) names the cause.
- Use `detailed` for per-phase explanation and the exact rule/ACE that matched.
- Nothing is really sent: it is a simulation, so it is safe in production and works even for flows that do not exist.
- The source must be reachable via the named ingress interface; only unicast protocols (tcp/udp/icmp) can be traced.

Source: Cisco ASA Series CLI Configuration Guide — https://www.cisco.com
