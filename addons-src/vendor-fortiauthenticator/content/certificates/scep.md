---
id = "ref/fortiauthenticator/scep"
kind = "configuration"
title = "SCEP enrollment service"
category = "reference/security/fortiauthenticator/certificates"
tags = ["fortiauthenticator", "identity", "scep", "certificates"]
version = "FortiAuthenticator 6.4"
---
FortiAuthenticator exposes an SCEP service so devices (mobile devices, firewalls, routers) can request and renew certificates automatically.

## Key pieces

- SCEP enrollment URL: the endpoint devices are provisioned with for automated certificate requests.
- Enrollment policies: which CA signs the requests, challenge/password settings and renewal windows.
- Approval flow: requests can auto-enroll or wait for administrator approval depending on policy.

## Notes

- SCEP simplifies large-scale deployment (MDM fleets) but a leaked challenge password allows certificate issuance — restrict distribution.
- Point devices at the enrollment URL and confirm issued certificates appear in the FAC request list.
- Pair SCEP enrollment with CRL publication so revoked device certificates fail validation downstream.

Source: Fortinet FortiAuthenticator Administration Guide — https://docs.fortinet.com
