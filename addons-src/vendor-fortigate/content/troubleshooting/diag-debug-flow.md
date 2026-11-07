---
id = "cmd/fortigate/diag-debug-flow"
kind = "command"
title = "diagnose debug flow"
category = "reference/vendors/fortinet/fortigate/troubleshooting"
tags = ["fortigate", "fortios", "cli", "debug"]
risk = "info"
version = "FortiOS 7.4"
---
Prints live packet decisions through the FortiGate: route lookup, policy search, match, NAT and profile handoff.

## Command

```cisco
diagnose debug reset
diagnose debug flow filter addr 10.0.0.5
diagnose debug flow trace start 10
diagnose debug enable
diagnose debug disable
```

## Notes

- filter addr restricts output to one host; filter port and filter proto are also available, trace start 10 traces 10 packets.
- The decisive lines are the policy search result (matched policy id) and any deny or no-route verdicts.
- Always run diagnose debug disable (and reset) when finished — debug output costs CPU on busy units.

Source: Fortinet FortiOS Administration Guide — https://docs.fortinet.com
