---
id = "ref/fortiauthenticator/radius-service"
kind = "configuration"
title = "RADIUS service"
category = "reference/security/fortiauthenticator/radius"
tags = ["fortiauthenticator", "identity", "radius"]
version = "FortiAuthenticator 6.4"
---
FortiAuthenticator runs a RADIUS service on UDP 1812/1813: NAS devices (clients) authenticate users against the local store or proxied directories.

## Key pieces

- RADIUS clients: every NAS (firewall, switch, VPN gateway) must be registered with its IP and shared secret.
- Authentication methods: PAP, CHAP, MS-CHAPv2 and EAP types for 802.1X, which require server certificates on FAC.
- Policies: match on user/group to assign authorization attributes (VLAN, filter-id) in the reply.

## Notes

- A wrong shared secret or unregistered NAS IP is the most common cause of silent rejections.
- EAP (802.1X) needs a certificate trusted by the endpoints; test with one client before fleet rollout.
- Check the live log view while reproducing a failed login — RADIUS rejections are logged with reason codes.

Source: Fortinet FortiAuthenticator Administration Guide — https://docs.fortinet.com
