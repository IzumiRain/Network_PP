---
id = "cmd/asa/nameif-security-level"
kind = "command"
title = "nameif and security-level (interface config)"
category = "reference/firewalls/cisco-asa/interfaces"
tags = ["asa", "cisco", "cli", "interfaces", "config"]
risk = "medium"
version = "ASA 9.16"
---
Names an interface and assigns its security level, which drives default trust between zones.

## Command

```cisco
interface GigabitEthernet0/1
 nameif inside
 security-level 100
 ip address 10.1.1.1 255.255.255.0
```

## Notes

- Defaults: inside = 100, outside = 0, others = 50; higher can reach lower unless blocked, lower cannot reach higher.
- Once an interface has an access-group ACL, the ACL (not the security level) decides traffic in that direction.
- Same-security traffic is denied between two equal-level interfaces unless `same-security-traffic permit inter-interface` is set.
- Removing `nameif` or changing the IP drops existing connections on that interface.

Source: Cisco ASA Series CLI Configuration Guide — https://www.cisco.com
