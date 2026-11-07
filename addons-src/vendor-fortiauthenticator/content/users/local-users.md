---
id = "ref/fortiauthenticator/local-users"
kind = "configuration"
title = "Local user management"
category = "reference/vendors/fortinet/fortiauthenticator/users"
tags = ["fortiauthenticator", "identity", "users"]
version = "FortiAuthenticator 6.4"
---
FortiAuthenticator can act as the primary identity store: local users with passwords, contact details for OTP delivery, and groups used for authorization.

## Key pieces

- Local users: username, password, email/phone used for one-time tokens, account expiry and lockout.
- User groups: carry the authorization decision (which services or VLANs a user gets) instead of per-user rules.
- Remote sync: local users can be provisioned by syncing from an upstream LDAP directory.

## Notes

- Keep authentication (FAC verifies identity) separate from authorization (what the network grants) — groups are the hook.
- Password policy and lockout settings protect against brute force on the most exposed credential store.
- When migrating away from a legacy RADIUS file store, import users and map groups before cutting over NAS devices.

Source: Fortinet FortiAuthenticator Administration Guide — https://docs.fortinet.com
