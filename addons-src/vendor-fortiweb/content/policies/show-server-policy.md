---
id = "cmd/fortiweb/show-server-policy"
kind = "command"
title = "show server-policy"
category = "reference/vendors/fortinet/fortiweb/policies"
tags = ["fortiweb", "waf", "cli", "policies"]
risk = "info"
version = "FortiWeb 7.2"
---
Prints the server policies: which interfaces and ports accept client traffic, which server pools answer, and which protection profile applies.

## Command

```cisco
show server-policy
```

## Notes

- The server policy is the traffic entry point; a request that never matches one never reaches WAF inspection.
- Check the frontend type (HTTP vs HTTPS) and certificate binding when clients fail TLS.
- The http-policy or inline protection profile field links the policy to its WAF enforcement.

Source: Fortinet FortiWeb Administration Guide — https://docs.fortinet.com
