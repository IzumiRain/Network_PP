---
id = "doc/fortinet/fortiauthenticator/fortiauthenticator"
kind = "vendor"
title = "Fortinet FortiAuthenticator"
category = "reference/vendors/fortinet/fortiauthenticator"
tags = ["fortiauthenticator", "identity", "api", "certificates", "pki", "scep", "ha", "ldap", "radius", "saml", "sso", "fido", "cli", "interfaces", "debug", "users", "model-doc"]
risk = "info"
version = "FortiAuthenticator 6.4 / FortiAuthenticator 6.4+"
---
The complete Fortinet FortiAuthenticator document: every topic for this model in one
file (v2 document model — see docs/PACK-SPEC.md, "Document model (v2 draft)").
Content is reorganised, unchanged, from this pack's original per-command
files; each topic is a section carrying its own tags via a section-tags
marker. Source attributions are kept per topic.


## API
<!-- section-tags: fortiauthenticator, identity, api -->


### REST API basics

FortiAuthenticator exposes a REST API under /api/v1/ over HTTPS for automating user provisioning, token management and queries.

#### Key pieces

- Base URL: https://<fac-address>/api/v1/ with JSON request/response bodies.
- Authentication: per-user API keys with defined permissions; enable API access and generate keys per integration account.
- Common uses: create/update local users, manage FortiToken assignments, and fetch user/group data for portals.

#### Notes

- Create a dedicated service account for API use instead of sharing an admin login.
- The API is the supported way to integrate FAC with ITSM/HR systems for joiner-leaver automation.
- Rate and audit the integration: every API change is an identity change, so log who did what.

Source: Fortinet FortiAuthenticator Administration Guide — https://docs.fortinet.com


## Certificates
<!-- section-tags: fortiauthenticator, identity, certificates, pki, scep -->


### Certificate management (CA, CSR, CRL)

FortiAuthenticator is a full CA: it can sign certificates, import CSRs, publish CRLs and manage trusted CAs used across the Fortinet estate.

#### Key pieces

- Root and intermediate CAs: FAC can generate them and sign subordinate/leaf certificates.
- CSRs: external requests are imported, signed and returned; certificate signing can also require approval.
- CRLs: revocation lists are published for distribution; revoked certificates stop validating once clients check them.
- Trusted CAs: imported CA certificates validate peers for EAP-TLS, SAML and other services.

#### Notes

- Protect the root CA: keep the root offline or on a dedicated hardened appliance and sign with an intermediate.
- The CA certificate on FAC is what 802.1X endpoints must trust; distribute it before enabling EAP.
- Track expiry dates of CA and CRL signing certificates — expiry breaks authentication fleet-wide.

Source: Fortinet FortiAuthenticator Administration Guide — https://docs.fortinet.com


### SCEP enrollment service

FortiAuthenticator exposes an SCEP service so devices (mobile devices, firewalls, routers) can request and renew certificates automatically.

#### Key pieces

- SCEP enrollment URL: the endpoint devices are provisioned with for automated certificate requests.
- Enrollment policies: which CA signs the requests, challenge/password settings and renewal windows.
- Approval flow: requests can auto-enroll or wait for administrator approval depending on policy.

#### Notes

- SCEP simplifies large-scale deployment (MDM fleets) but a leaked challenge password allows certificate issuance — restrict distribution.
- Point devices at the enrollment URL and confirm issued certificates appear in the FAC request list.
- Pair SCEP enrollment with CRL publication so revoked device certificates fail validation downstream.

Source: Fortinet FortiAuthenticator Administration Guide — https://docs.fortinet.com


## HA
<!-- section-tags: fortiauthenticator, identity, ha -->


### Clustering (HA)

FortiAuthenticator clusters provide HA for identity services: a primary node replicates configuration and user data to secondary (and optional tertiary) members.

#### Key pieces

- Cluster roles: primary handles writes and synchronization; secondary/tertiary serve authentication and take over on failure.
- Synchronization: configuration and certificate/user data replicate over the cluster link; keep time synced across members.
- NAS devices: point RADIUS/LDAP clients at both members or at a VIP so failover is transparent.

#### Notes

- An out-of-sync cluster silently serves stale users and certificates; check sync state after every change.
- Certificates are cluster-shared, but verify the SANs cover the cluster name/VIP used by clients.
- Test failover quarterly: stop the primary and confirm RADIUS and SSO services continue on the secondary.

Source: Fortinet FortiAuthenticator Administration Guide — https://docs.fortinet.com


## LDAP
<!-- section-tags: fortiauthenticator, identity, ldap -->


### LDAP service

FortiAuthenticator can also expose an LDAP service so directory-aware devices (firewalls, VPNs, other appliances) can bind and authenticate against it.

#### Key pieces

- LDAP service endpoint: standard directory port, with TLS available for secure binds.
- Bind users: clients authenticate with a bind DN/password or simple binds from the actual end users.
- Search scope: base DN and filters decide which part of the local or synced directory clients can query.

#### Notes

- Restrict who can bind: unauthenticated or overly broad binds leak user enumeration to the network.
- Configure the client (for example a FortiGate user group of type LDAP) with the same base DN and bind credentials you set here.
- Use TLS binds in production; plaintext LDAP transmits credentials in the clear.

Source: Fortinet FortiAuthenticator Administration Guide — https://docs.fortinet.com


## Radius
<!-- section-tags: fortiauthenticator, identity, radius -->


### RADIUS service

FortiAuthenticator runs a RADIUS service on UDP 1812/1813: NAS devices (clients) authenticate users against the local store or proxied directories.

#### Key pieces

- RADIUS clients: every NAS (firewall, switch, VPN gateway) must be registered with its IP and shared secret.
- Authentication methods: PAP, CHAP, MS-CHAPv2 and EAP types for 802.1X, which require server certificates on FAC.
- Policies: match on user/group to assign authorization attributes (VLAN, filter-id) in the reply.

#### Notes

- A wrong shared secret or unregistered NAS IP is the most common cause of silent rejections.
- EAP (802.1X) needs a certificate trusted by the endpoints; test with one client before fleet rollout.
- Check the live log view while reproducing a failed login — RADIUS rejections are logged with reason codes.

Source: Fortinet FortiAuthenticator Administration Guide — https://docs.fortinet.com


## SSO
<!-- section-tags: fortiauthenticator, identity, saml, sso, fido -->


### SSO services and FIDO

FortiAuthenticator provides SSO services: it acts as a SAML2 identity provider or service provider and as an SSO portal, and supports FIDO2 security keys as an MFA method for local users in recent releases.

#### Key pieces

- SAML2 IdP: FAC authenticates users (password + MFA) and issues assertions to SPs such as FortiGate SSL-VPN or SaaS apps.
- SAML2 SP: FAC can also consume assertions from an external IdP for delegated login.
- SSO portal: a single portal page that federates users into multiple registered services.
- FIDO2 keys: hardware security keys registered per local user for phishing-resistant second factor.

#### Notes

- SAML integration with FortiGate requires matching entity IDs and certificate trust on both sides; clock skew breaks assertions.
- Register FIDO2 keys per user and keep recovery factors (OTP or admin reset) for lost keys.
- Test SP metadata import/export after every certificate change — stale signing certs are the usual SSO failure.

Source: Fortinet FortiAuthenticator Administration Guide — https://docs.fortinet.com


## System
<!-- section-tags: fortiauthenticator, identity, cli, interfaces -->


### get system status

Shows the FortiAuthenticator appliance summary: firmware version, model, serial number and uptime.

#### Command

```cisco
get system status
```

#### Notes

- Confirms firmware build and licensing state before you change identity services.
- Serial number is required for FortiCare cases and cluster member identification.
- An identity appliance outage takes down MFA for everything behind it, so check status after reboots first.

Source: Fortinet FortiAuthenticator Administration Guide — https://docs.fortinet.com


### show system global

Dumps the global system configuration: hostname, admin settings, DNS and NTP-related platform options.

#### Command

```cisco
show system global
```

#### Notes

- Correct NTP and DNS matter more than usual here: certificates, SAML and OTP all depend on accurate time.
- Check the admin HTTPS port and timeout before scripted or firewalled GUI access.
- Read-only view; changes are made under config system global.

Source: Fortinet FortiAuthenticator Administration Guide — https://docs.fortinet.com


### show system interface

Prints interface configuration: IP address, mode, speed/duplex and which management services are allowed.

#### Command

```cisco
show system interface
show system interface port1
```

#### Notes

- Verify the IP and allowaccess flags when devices cannot reach the RADIUS (1812) or LDAP services.
- Identity appliances often need fixed addressing; DHCP interfaces on an auth server are a red flag.
- Pair with upstream routing checks if remote NAS devices time out against this appliance.

Source: Fortinet FortiAuthenticator Administration Guide — https://docs.fortinet.com


## Troubleshooting
<!-- section-tags: fortiauthenticator, identity, cli, debug -->


### diagnose debug enable and disable

Toggles CLI debug output for the FortiOS-style diagnose debug commands on the appliance.

#### Command

```cisco
diagnose debug enable
diagnose debug disable
```

#### Notes

- Enable debug output before running targeted debug commands, then disable immediately afterwards.
- Most day-to-day FAC troubleshooting uses the GUI: live logs under Logging, RADIUS/LDAP failure counters and the certificate pages.
- Debug sessions on a busy identity appliance add CPU load; avoid leaving them running.

Source: Fortinet FortiAuthenticator Administration Guide — https://docs.fortinet.com


## Users
<!-- section-tags: fortiauthenticator, identity, users -->


### Local user management

FortiAuthenticator can act as the primary identity store: local users with passwords, contact details for OTP delivery, and groups used for authorization.

#### Key pieces

- Local users: username, password, email/phone used for one-time tokens, account expiry and lockout.
- User groups: carry the authorization decision (which services or VLANs a user gets) instead of per-user rules.
- Remote sync: local users can be provisioned by syncing from an upstream LDAP directory.

#### Notes

- Keep authentication (FAC verifies identity) separate from authorization (what the network grants) — groups are the hook.
- Password policy and lockout settings protect against brute force on the most exposed credential store.
- When migrating away from a legacy RADIUS file store, import users and map groups before cutting over NAS devices.

Source: Fortinet FortiAuthenticator Administration Guide — https://docs.fortinet.com

