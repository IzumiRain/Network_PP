---
id = "cmd/asa/show-vpn-sessiondb"
kind = "command"
title = "show vpn-sessiondb"
category = "reference/firewalls/cisco-asa/vpn"
tags = ["asa", "cisco", "cli", "vpn"]
risk = "info"
version = "ASA 9.16"
---
VPN session database: summary, per-user (AnyConnect) and site-to-site tunnel details.

## Command

```cisco
show vpn-sessiondb summary
show vpn-sessiondb anyconnect
show vpn-sessiondb l2l
show vpn-sessiondb anyconnect filter name jdoe
```

## Notes

- `summary` is the first stop for counts by type and license/limit usage.
- `l2l` entries show the peer IP, tunnel group and assigned networks per site-to-site tunnel.
- `filter name <user>` and `sort by` narrow long user lists during a support call.
- Session limits hitting capacity show here as denied session attempts before users report failures.

Source: Cisco ASA Series CLI Configuration Guide — https://www.cisco.com
