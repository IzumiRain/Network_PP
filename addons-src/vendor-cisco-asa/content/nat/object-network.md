---
id = "cmd/asa/object-network"
kind = "command"
title = "object network (NAT/ACL objects)"
category = "reference/firewalls/cisco-asa/nat"
tags = ["asa", "cisco", "cli", "nat", "config"]
risk = "medium"
version = "ASA 9.16"
---
Defines reusable network objects (host, subnet, range) used by NAT rules and ACLs.

## Command

```cisco
object network WEB_SERVER
 host 10.2.2.10
object network WEB_NET
 subnet 10.2.2.0 255.255.255.0
nat (inside,outside) static 203.0.113.10
```

## Notes

- Objects must be created before they are referenced by `object network OBJ` NAT or `object-group`/ACL entries.
- A `nat` statement inside `object network` creates object (section 2) auto NAT.
- Deleting or editing an object referenced by NAT/ACLs changes live policy immediately - change windows apply.
- `show run object` dumps all object definitions for review.

Source: Cisco ASA Series CLI Configuration Guide — https://www.cisco.com
