---
id = "ref/f5-bigip/irules-basics"
kind = "configuration"
title = "iRules basics"
category = "reference/vendors/f5/bigip/irules"
tags = ["f5", "bigip", "irules", "tcl"]
version = "BIG-IP 16.1"
---
iRules are TCL scripts attached to virtual servers that inspect and steer traffic at defined processing events.

## Structure

```tcl
when HTTP_REQUEST {
    if { [HTTP::host] eq "old.example.com" } {
        HTTP::redirect "https://www.example.com[HTTP::uri]"
    }
}
```

- Events: CLIENT_ACCEPTED (connection), HTTP_REQUEST/HTTP_RESPONSE (layer 7) are the most used.
- Commands: pool selects a pool, HTTP::redirect/HTTP::header rewrite requests and responses, IP::client_addr reads the source.

## Notes

- Keep rules small and event-scoped; rules run per event on live traffic and cost CPU.
- Log with log local0. during development, then remove verbose logging before production.
- List and review with tmsh list ltm rule; prefer profiles and policy features when an iRule is only doing simple matching.

Source: F5 iRules and tmsh Reference Guide — https://techdocs.f5.com
