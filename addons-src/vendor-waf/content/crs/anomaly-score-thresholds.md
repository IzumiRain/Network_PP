---
id = "ref/waf/crs-anomaly-score-thresholds"
kind = "protocol"
title = "Anomaly score thresholds"
category = "reference/security/waf/crs"
tags = ["waf", "crs", "anomaly-scoring", "tuning"]
version = "OWASP CRS 3.x/4.x"
---
Inbound and outbound score thresholds decide when a transaction is blocked under anomaly scoring.

## Example

```apache
# crs-setup.conf (defaults)
# setvar:tx.inbound_anomaly_score_threshold=5
# setvar:tx.outbound_anomaly_score_threshold=4
```

## Notes

- Default thresholds are 5 (inbound) and 4 (outbound); each matching CRS rule adds its own score contribution.
- Raise thresholds for sites with unavoidable partial matches; lower them to block sooner at the cost of false positives.
- The final evaluation rules 949110 (inbound) and 959100 (outbound) fire at the threshold and produce the block decision.
- Thresholds work together with paranoia level: raising PL adds rules and their scores, which can raise effective strictness twice.

Source: OWASP Core Rule Set Documentation — https://coreruleset.org
