---
id = "err/network-common"
kind = "error"
title = "Common Network Error Messages"
category = "reference/errors/network"
tags = ["errors", "troubleshooting"]
risk = "info"
---
Decoded: what these actually mean and where to look next.

## Messages
<!-- section-tags: errors, troubleshooting -->

| Message | Real meaning | Next step |
|---|---|---|
| `Destination host unreachable` | no route to the host (from your stack or a router) | check gateway + routes; whose IP is in the reply? |
| `Request timed out` | sent, nothing came back | host down, firewall dropping, or wrong path |
| `TTL expired in transit` | routing loop | traceroute, check for route flapping |
| `TTL expired in reassembly` | fragments never completed | MTU/fragmentation filtering |
| `General failure` (Windows) | request never left the stack | NIC/driver, no valid IP config |
| `connection refused` (ECONNREFUSED) | host answered: nothing listens on that port | service down? right port? |
| `connection timed out` (ETIMEDOUT) | no answer at all | firewall drop or routing black hole |
| `network is unreachable` (ENETUNREACH) | no route in the local table | default gateway missing |
| `No such device / Network down` | interface problem | driver, cable, `ip link` state |
| `certificate has expired` | TLS validation failed at date check | renew / fix clock |
| `hostname mismatch` | cert doesn't cover the name used | SNI / correct name / reissue |
| `NXDOMAIN` | name doesn't exist upstream | typo or DNS path issue |
| `SERVFAIL` | resolver couldn't get an answer | upstream chain broken; try another resolver |
| `DHCP lease renewal failed` | server unreachable at T1 | eventually APIPA; check relay |

Pattern: refused = the host spoke; timed out = silence (usually a firewall);
unreachable = nobody knows the way.
