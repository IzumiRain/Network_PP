---
id = "cmd/fortigate/show-firewall-policy"
kind = "command"
title = "show firewall policy"
category = "reference/firewalls/fortigate/policies"
tags = ["fortigate", "fortios", "cli", "policies"]
risk = "info"
version = "FortiOS 7.4"
---
Dumps the firewall policy table in evaluation order, including source/destination interfaces, addresses, services, NAT and profiles.

## Command

```cisco
show firewall policy
show firewall policy 12
```

## Notes

- Policies are evaluated top-down and the first match wins, so output order is the processing order.
- With a policy id the command prints a single rule — use the id seen in debug flow output.
- Verify srcintf/dstintf pairs: a rule never matches if traffic enters/leaves other interfaces.

Source: Fortinet FortiOS Administration Guide — https://docs.fortinet.com
