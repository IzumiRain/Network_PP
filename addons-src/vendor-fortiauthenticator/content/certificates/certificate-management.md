---
id = "ref/fortiauthenticator/certificate-management"
kind = "configuration"
title = "Certificate management (CA, CSR, CRL)"
category = "reference/vendors/fortinet/fortiauthenticator/certificates"
tags = ["fortiauthenticator", "identity", "certificates", "pki"]
version = "FortiAuthenticator 6.4"
---
FortiAuthenticator is a full CA: it can sign certificates, import CSRs, publish CRLs and manage trusted CAs used across the Fortinet estate.

## Key pieces

- Root and intermediate CAs: FAC can generate them and sign subordinate/leaf certificates.
- CSRs: external requests are imported, signed and returned; certificate signing can also require approval.
- CRLs: revocation lists are published for distribution; revoked certificates stop validating once clients check them.
- Trusted CAs: imported CA certificates validate peers for EAP-TLS, SAML and other services.

## Notes

- Protect the root CA: keep the root offline or on a dedicated hardened appliance and sign with an intermediate.
- The CA certificate on FAC is what 802.1X endpoints must trust; distribute it before enabling EAP.
- Track expiry dates of CA and CRL signing certificates — expiry breaks authentication fleet-wide.

Source: Fortinet FortiAuthenticator Administration Guide — https://docs.fortinet.com
