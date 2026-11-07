---
id = "ref/fortiauthenticator/clustering"
kind = "configuration"
title = "Clustering (HA)"
category = "reference/vendors/fortinet/fortiauthenticator/ha"
tags = ["fortiauthenticator", "identity", "ha"]
version = "FortiAuthenticator 6.4"
---
FortiAuthenticator clusters provide HA for identity services: a primary node replicates configuration and user data to secondary (and optional tertiary) members.

## Key pieces

- Cluster roles: primary handles writes and synchronization; secondary/tertiary serve authentication and take over on failure.
- Synchronization: configuration and certificate/user data replicate over the cluster link; keep time synced across members.
- NAS devices: point RADIUS/LDAP clients at both members or at a VIP so failover is transparent.

## Notes

- An out-of-sync cluster silently serves stale users and certificates; check sync state after every change.
- Certificates are cluster-shared, but verify the SANs cover the cluster name/VIP used by clients.
- Test failover quarterly: stop the primary and confirm RADIUS and SSO services continue on the secondary.

Source: Fortinet FortiAuthenticator Administration Guide — https://docs.fortinet.com
