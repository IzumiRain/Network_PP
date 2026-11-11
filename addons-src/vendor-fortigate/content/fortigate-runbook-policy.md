---
id = "doc/fortinet/fortigate/runbook-policy-not-matching"
kind = "runbook"
title = "Runbook: FortiGate firewall policy not matching"
category = "runbooks/vendors/fortinet/fortigate"
tags = ["fortigate", "fortios", "runbook", "policies", "model-doc"]
version = "FortiOS 7.4"
---
v2 pilot runbook document (see docs/PACK-SPEC.md, "Document model (v2
draft)"): the runbook as ONE document for the model, its steps as tagged
sections. Content is the existing `rb/fortigate/policy-not-matching`
runbook, reorganised unchanged; the original file remains in place during
migration.

## Policy not matching — debug flow
<!-- section-tags: runbook, fortigate, policies, debug-flow -->

Traffic lands on the implicit deny (or the wrong rule) although a policy seems to cover it. Follow the debug flow decision path.

#### Steps

```cisco
diagnose debug reset
diagnose debug flow filter addr 10.0.0.5
diagnose debug flow trace start 10
diagnose debug enable
```

#### Read the output

- No route found for the destination: fix routing first (get router info routing-table details 203.0.113.10).
- Policy search matched id=N: inspect that rule with show firewall policy N — it may be the wrong one.
- Deny with no policy id: nothing matched, so check srcintf/dstintf, address objects and service in the intended rule.
- Wrong rule matched: policies are top-down first-match, so reorder or narrow the earlier rule.

#### Wrap up

- Stop with diagnose debug disable, then clear session filters you set during the investigation.

Source: Fortinet FortiOS Administration Guide — https://docs.fortinet.com
