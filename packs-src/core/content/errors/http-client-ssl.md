---
id = "err/tls-common"
kind = "error"
title = "TLS / SSL Error Messages"
category = "reference/errors/tls"
tags = ["tls", "errors", "certificates"]
risk = "info"
---
What TLS errors in clients and proxies actually point at.

## Messages
<!-- section-tags: tls, errors, certificates -->

| Error | Cause | Fix direction |
|---|---|---|
| `certificate has expired` | Not After passed | renew cert; also check system clock |
| `certificate is not yet valid` | Not Before in the future | clock skew on client or server |
| `unable to get local issuer certificate` | missing intermediate | install full chain (leaf → intermediates → root) |
| `hostname mismatch` / `IP mismatch` | SAN doesn't cover the name | use a covered name or reissue with right SAN |
| `self-signed certificate` | chain anchors nowhere | trust it explicitly (labs) or use real CA |
| `unknown ca` | client doesn't trust the issuer | add CA to trust store |
| `certificate verify failed` (curl) | umbrella for the above | `openssl s_client` shows the real reason |
| `handshake failure` | no shared cipher/version | server too old/new; align TLS versions |
| `protocol version` errors | client speaks TLS1.3, server wants 1.0… | legacy system — isolate, don't downgrade globally |
| `wrong version number` | you spoke TLS to a plain HTTP port | wrong port — check 443 vs 80 |
| `connection reset during handshake` | middlebox or server policy | firewall/IPS inspecting; try from another path |
| `EOSSL` / unexpected EOF | connection cut mid-handshake | MTU on VPNs is a classic culprit |

Debug ladder: `openssl s_client -connect host:443 -servername host` → read
the chain, dates, SANs, verify return code; then `curl -vI https://host` to
compare what a second stack says.
