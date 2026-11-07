---
id = "ref/f5-bigip/snat"
kind = "configuration"
title = "SNAT and NAT concepts"
category = "reference/devices/f5-bigip/nat"
tags = ["f5", "bigip", "snat", "nat"]
version = "BIG-IP 16.1"
---
On BIG-IP, source translation decides what source IP the pool member sees: none (client IP preserved), automap (self IP) or an SNAT pool.

## Key pieces

- SNAT automap: translates the client source to the BIG-IP self IP, guaranteeing return traffic passes the unit.
- SNAT pool: translates to a dedicated NAT address range — needed when pools sit on networks without a route back via the BIG-IP.
- Fixed NAT (ltm nat): static one-to-one mappings, configured rarely compared to SNAT pools.
- Set per virtual server via source-address-translation (see tmsh list ltm virtual), with pools listed by tmsh list ltm snatpool.

## Notes

- Skipping SNAT on asymmetric setups breaks connections because return traffic bypasses the BIG-IP.
- Automap hides client IPs from servers — log client IPs in X-Forwarded-For when servers need them.
- SNAT ports are finite: high-connection virtuals need adequate SNAT pool sizing.

Source: F5 BIG-IP LTM Concepts Guide — https://techdocs.f5.com
