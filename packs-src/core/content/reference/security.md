---
id = "doc/networking/security/security"
kind = "protocol"
title = "Networking: Security"
category = "reference/networking/security"
tags = ["tls", "security", "https", "model-doc"]
risk = "info"
---
The complete Networking: Security document: every topic for this area in one
file (v2 document model — see docs/PACK-SPEC.md, "Document model (v2 draft)").
Content is reorganised, unchanged, from this pack's original per-topic
files; each topic is a section carrying its own tags via a section-tags
marker.

## TLS / SSL — what the handshake proves
<!-- section-tags: tls, security, https -->

Version: RFC 8446 (TLS 1.3)

TLS wraps a transport connection in encryption + authentication + integrity.
HTTPS = HTTP inside TLS on port 443.

#### What the handshake does

1. Agree on a cipher suite and TLS version
2. Server presents its **certificate** (identity bound to a public key)
3. Client validates the chain to a trusted root, checks the hostname and
   expiry
4. Keys are derived (ECDHE — forward secret); application data starts

#### Field checklist when TLS fails

- Expired certificate — by far #1 (check Not After)
- Hostname mismatch — cert CN/SAN doesn't cover the name you used
- Missing intermediate — works in some browsers, fails in others/curl
- Wrong SNI → server presents default cert
- Old TLS version required by legacy gear — reject; don't downgrade

`openssl s_client -connect host:443 -servername host` remains the universal
probe; the certificate chain, dates and SANs are all in its output.

