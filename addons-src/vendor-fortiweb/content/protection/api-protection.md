---
id = "ref/fortiweb/api-protection"
kind = "configuration"
title = "API protection"
category = "reference/vendors/fortinet/fortiweb/protection"
tags = ["fortiweb", "waf", "api"]
version = "FortiWeb 7.2"
---
API protection enforces a strict contract on JSON/XML traffic: only requests matching the declared API schema are forwarded, everything else is logged or blocked.

## Key pieces

- Schema import: OpenAPI (Swagger) or manually declared paths, methods and parameter types.
- Validation: FortiWeb rejects unknown fields, wrong types and out-of-spec requests before they reach the backend.
- Rate limiting and authentication offload are typically layered on the same API rules.

## Notes

- The schema must track the application: every legitimate API change needs a FortiWeb update or traffic gets blocked.
- Start in alert-only mode to measure drift between schema and real traffic.
- API rules bind through the protection profile like other WAF features.

Source: Fortinet FortiWeb Administration Guide — https://docs.fortinet.com
