---
id = "rb/fortigate/policy-not-matching"
kind = "runbook"
title = "Runbook: firewall policy not matching"
category = "runbooks/fortigate"
tags = ["fortigate", "fortios", "runbook", "policies"]
version = "FortiOS 7.4"
---
Traffic lands on the implicit deny (or the wrong rule) although a policy seems to cover it. Follow the debug flow decision path.

## Steps

```cisco
diagnose debug reset
diagnose debug flow filter addr 10.0.0.5
diagnose debug flow trace start 10
diagnose debug enable
```

## Read the output

- No route found for the destination: fix routing first (get router info routing-table details 203.0.113.10).
- Policy search matched id=N: inspect that rule with show firewall policy N — it may be the wrong one.
- Deny with no policy id: nothing matched, so check srcintf/dstintf, address objects and service in the intended rule.
- Wrong rule matched: policies are top-down first-match, so reorder or narrow the earlier rule.

## Wrap up

- Stop with diagnose debug disable, then clear session filters you set during the investigation.

Source: Fortinet FortiOS Administration Guide — https://docs.fortinet.com
