---
id = "cmd/fortigate/diag-test-application-dnsserver"
kind = "command"
title = "diagnose test application dnsserver"
category = "reference/vendors/fortinet/fortigate/system"
tags = ["fortigate", "fortios", "cli", "dns"]
risk = "info"
version = "FortiOS 7.4"
---
Runs a live query test against the configured DNS servers and reports the response time per server.

## Command

```cisco
diagnose test application dnsserver 3
```

## Notes

- The option value selects the test action; 3 queries each configured DNS server and prints its RTT.
- Use it when DNS-dependent features (web filtering, FortiGuard lookups) are slow or timing out.
- Requires DNS servers set under config system dns; a missing RTT means that server is unreachable.

Source: Fortinet FortiOS CLI Reference — https://docs.fortinet.com
