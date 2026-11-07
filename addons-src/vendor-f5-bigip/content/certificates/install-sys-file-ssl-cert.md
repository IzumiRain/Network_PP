---
id = "cmd/f5-bigip/install-sys-file-ssl-cert"
kind = "command"
title = "install sys file ssl-cert"
category = "reference/vendors/f5/bigip/certificates"
tags = ["f5", "bigip", "tmsh", "certificates", "ssl"]
risk = "medium"
version = "BIG-IP 16.1"
---
Imports an SSL certificate into the BIG-IP certificate store from a local file on the appliance.

## Command

```bash
tmsh install sys file ssl-cert /Common/example.crt from-local-file /var/tmp/example.crt
```

## Notes

- Copy the certificate to /var/tmp first (scp), then install it into the /Common partition.
- The certificate then appears in the store and can be attached to client-ssl profiles on virtual servers.
- Replacing an expired cert is straightforward; test the affected client-ssl profile afterwards in a browser.

Source: F5 BIG-IP tmsh Reference Guide — https://techdocs.f5.com
