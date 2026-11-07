---
id = "ref/fortiauthenticator/sso-services"
kind = "configuration"
title = "SSO services and FIDO"
category = "reference/security/fortiauthenticator/sso"
tags = ["fortiauthenticator", "identity", "saml", "sso", "fido"]
version = "FortiAuthenticator 6.4+"
---
FortiAuthenticator provides SSO services: it acts as a SAML2 identity provider or service provider and as an SSO portal, and supports FIDO2 security keys as an MFA method for local users in recent releases.

## Key pieces

- SAML2 IdP: FAC authenticates users (password + MFA) and issues assertions to SPs such as FortiGate SSL-VPN or SaaS apps.
- SAML2 SP: FAC can also consume assertions from an external IdP for delegated login.
- SSO portal: a single portal page that federates users into multiple registered services.
- FIDO2 keys: hardware security keys registered per local user for phishing-resistant second factor.

## Notes

- SAML integration with FortiGate requires matching entity IDs and certificate trust on both sides; clock skew breaks assertions.
- Register FIDO2 keys per user and keep recovery factors (OTP or admin reset) for lost keys.
- Test SP metadata import/export after every certificate change — stale signing certs are the usual SSO failure.

Source: Fortinet FortiAuthenticator Administration Guide — https://docs.fortinet.com
