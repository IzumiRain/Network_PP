---
id = "cmd/f5-bigip/list-ltm-virtual"
kind = "command"
title = "list ltm virtual"
category = "reference/devices/f5-bigip/ltm"
tags = ["f5", "bigip", "tmsh", "ltm"]
risk = "info"
version = "BIG-IP 16.1"
---
Prints the LTM virtual server configuration: destination, port, pool, profiles, iRules and SNAT settings.

## Command

```bash
tmsh list ltm virtual
tmsh list ltm virtual /Common/vs-app-443
```

## Notes

- The virtual server is the traffic entry point; verify destination/port and pool first when a service is down.
- Check the source-address-translation field here to see whether and how SNAT is applied.
- The profiles list (tcp, http, ssl client/server) explains most protocol-level behavior differences.

Source: F5 BIG-IP tmsh Reference Guide — https://techdocs.f5.com
