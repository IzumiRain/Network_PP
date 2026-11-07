---
id = "ref/fortiauthenticator/ldap-service"
kind = "configuration"
title = "LDAP service"
category = "reference/security/fortiauthenticator/ldap"
tags = ["fortiauthenticator", "identity", "ldap"]
version = "FortiAuthenticator 6.4"
---
FortiAuthenticator can also expose an LDAP service so directory-aware devices (firewalls, VPNs, other appliances) can bind and authenticate against it.

## Key pieces

- LDAP service endpoint: standard directory port, with TLS available for secure binds.
- Bind users: clients authenticate with a bind DN/password or simple binds from the actual end users.
- Search scope: base DN and filters decide which part of the local or synced directory clients can query.

## Notes

- Restrict who can bind: unauthenticated or overly broad binds leak user enumeration to the network.
- Configure the client (for example a FortiGate user group of type LDAP) with the same base DN and bind credentials you set here.
- Use TLS binds in production; plaintext LDAP transmits credentials in the clear.

Source: Fortinet FortiAuthenticator Administration Guide — https://docs.fortinet.com
