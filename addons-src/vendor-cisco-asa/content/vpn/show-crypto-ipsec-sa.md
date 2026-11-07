---
id = "cmd/asa/show-crypto-ipsec-sa"
kind = "command"
title = "show crypto ipsec sa"
category = "reference/vendors/cisco/asa/vpn"
tags = ["asa", "cisco", "cli", "vpn", "ipsec"]
risk = "info"
version = "ASA 9.16"
---
IPsec SA details: phase-2 proposals, encaps/decaps counters and interesting-traffic ACL.

## Command

```cisco
show crypto ipsec sa
show crypto ipsec sa peer 203.0.113.5
show crypto ipsec sa detail
```

## Notes

- Rising encap AND decap counters prove live two-way traffic; encap without decap usually means a return-path/ACL/NAT issue.
- `pkts encaps`/`pkts decaps` near zero with an active SA means no interesting traffic is matching.
- Rekey attempts and errors (e.g. `sa created`, `route-metric`) show in the peer block.
- Old crypto-map L2L designs gate phase 2 on a crypto ACL; mismatched networks on either side show as no-SA.

Source: Cisco ASA Series Command Reference — https://www.cisco.com
