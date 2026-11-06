---
id = "rb/net/dns-resolution-failure"
kind = "runbook"
title = "Runbook: DNS resolution failures"
category = "runbooks/network"
tags = ["dns", "troubleshooting"]
risk = "info"
version = "core-0.2.0"
---
## Prerequisites

- A failing name and a working name (pick any public site)
- The list of resolvers the host is configured to use

## Steps

- [ ] 1. Confirm it is DNS: `ping 8.8.8.8` works while `ping example.com` fails
- [ ] 2. Read the resolver config: `resolv.conf` / `Get-DnsClientServerAddress` / `scutil --dns`
- [ ] 3. Query the configured resolver directly: `dig @<resolver> example.com` — works? then the local stub/cache layer is the problem (flush cache)
- [ ] 4. Query a public resolver: `dig @1.1.1.1 example.com` — works while step 3 fails → the configured resolver is down or unreachable
- [ ] 5. Compare answers: different IPs from different resolvers → split-horizon, geo-DNS, or a stale static entry
- [ ] 6. Check the error type: NXDOMAIN (name really missing / search-domain appended), SERVFAIL (upstream chain broken), REFUSED (policy or recursion disabled)
- [ ] 7. For internal names: check the zone on the authoritative server and DHCP-issued search domain
- [ ] 8. Check for a captive portal / VPN DNS override hijacking port 53

## Validation

- [ ] Internal and external names both resolve
- [ ] Negative caching cleared (previously-failing names answer)
- [ ] Latency sane (< 100ms for cached, < 500ms cold)

## Decisions

- All resolvers fail from this host only → local path/firewall
- Fail from all hosts → resolver service or upstream connectivity
- Only internal names fail → authoritative zone or forwarding config
