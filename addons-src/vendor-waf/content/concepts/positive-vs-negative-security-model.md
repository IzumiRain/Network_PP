---
id = "ref/waf/positive-vs-negative-security-model"
kind = "protocol"
title = "Positive vs negative security model"
category = "reference/security/waf/concepts"
tags = ["waf", "concepts", "security-model"]
version = "WAF concepts (CRS 3.x/4.x)"
---
A WAF either blocks what looks bad (negative security) or only allows what is known good (positive security).

## Comparison

| Model | Logic | Strength | Weakness |
|---|---|---|---|
| Negative | Block signatures of known attacks | Quick to deploy, few breakages | Blind to novel attacks |
| Positive | Allow only expected methods, parameters and formats | Blocks unknown attacks too | Heavy per-app tuning effort |

## Notes

- Signature-driven rule sets like OWASP CRS are negative-model with anomaly scoring on top.
- Positive rules are written per endpoint: allowed HTTP method, parameter names, value types and lengths.
- Production deployments usually layer both: positive rules on critical endpoints, CRS as the negative baseline.
- Start with the negative model in DetectionOnly, then add positive rules where the risk justifies the tuning cost.

Source: OWASP WAF / AppSec documentation — https://owasp.org
