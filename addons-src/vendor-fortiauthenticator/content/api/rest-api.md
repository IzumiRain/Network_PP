---
id = "ref/fortiauthenticator/rest-api"
kind = "configuration"
title = "REST API basics"
category = "reference/vendors/fortinet/fortiauthenticator/api"
tags = ["fortiauthenticator", "identity", "api"]
version = "FortiAuthenticator 6.4"
---
FortiAuthenticator exposes a REST API under /api/v1/ over HTTPS for automating user provisioning, token management and queries.

## Key pieces

- Base URL: https://<fac-address>/api/v1/ with JSON request/response bodies.
- Authentication: per-user API keys with defined permissions; enable API access and generate keys per integration account.
- Common uses: create/update local users, manage FortiToken assignments, and fetch user/group data for portals.

## Notes

- Create a dedicated service account for API use instead of sharing an admin login.
- The API is the supported way to integrate FAC with ITSM/HR systems for joiner-leaver automation.
- Rate and audit the integration: every API change is an identity change, so log who did what.

Source: Fortinet FortiAuthenticator Administration Guide — https://docs.fortinet.com
