---
id = "cmd/f5-bigip/list-ltm-rule"
kind = "command"
title = "list ltm rule (iRules)"
category = "reference/devices/f5-bigip/ltm"
tags = ["f5", "bigip", "tmsh", "irules"]
risk = "info"
version = "BIG-IP 16.1"
---
Prints iRule definitions written in TCL: the events they hook and the logic applied to traffic.

## Command

```bash
tmsh list ltm rule
tmsh list ltm rule /Common/ir-redirect-http
```

## Notes

- iRules execute on events such as CLIENT_ACCEPTED, HTTP_REQUEST or HTTP_RESPONSE and can select pools, rewrite headers or redirect.
- Check which virtuals reference a rule before editing it — one rule often serves many applications.
- A buggy iRule on a hot path can stall all traffic on the virtual; test changes in a staging environment first.

Source: F5 BIG-IP tmsh Reference Guide — https://techdocs.f5.com
