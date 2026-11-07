---
id = "ref/waf/blocking-vs-anomaly-mode"
kind = "protocol"
title = "CRS modes: traditional blocking vs anomaly scoring"
category = "reference/security/waf/crs"
tags = ["waf", "crs", "anomaly-scoring"]
version = "OWASP CRS 3.x/4.x"
---
The CRS can block per matched rule (traditional mode) or accumulate scores and block at a threshold (anomaly scoring, the default since CRS 3.x).

## Example

```apache
# crs-setup.conf - traditional mode is OFF by default; both lines uncomment it
#SecDefaultAction "phase:1,deny,log,status:403"
#SecDefaultAction "phase:2,deny,log,status:403"
```

## Notes

- Keep the SecDefaultAction lines commented to stay in anomaly scoring mode (recommended and default).
- Traditional mode blocks on the first matching rule, which loses score detail and makes self-matching rules disrupt traffic.
- In anomaly scoring the final evaluation rules (949110 inbound, 959100 outbound) apply the configured thresholds.
- Never run CRS 3.x+ in traditional mode for a new deployment; older CRS 2.x setups are the main reason the mode still exists.

Source: OWASP Core Rule Set Documentation — https://coreruleset.org
