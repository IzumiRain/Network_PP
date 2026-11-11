---
id = "doc/runbooks/network"
kind = "runbook"
title = "Runbooks: Network"
category = "runbooks/network"
tags = ["dns", "troubleshooting", "network", "connectivity", "model-doc"]
risk = "info"
version = "core-0.3.0"
---
Every network runbook in one document (v2 document model — see
docs/PACK-SPEC.md). Each runbook is a section carrying its own tags.


## DNS resolution failures
<!-- section-tags: dns, troubleshooting -->

#### Prerequisites

- A failing name and a working name (pick any public site)
- The list of resolvers the host is configured to use

#### Steps

- [ ] 1. Confirm it is DNS: `ping 8.8.8.8` works while `ping example.com` fails
- [ ] 2. Read the resolver config: `resolv.conf` / `Get-DnsClientServerAddress` / `scutil --dns`
- [ ] 3. Query the configured resolver directly: `dig @<resolver> example.com` — works? then the local stub/cache layer is the problem (flush cache)
- [ ] 4. Query a public resolver: `dig @1.1.1.1 example.com` — works while step 3 fails → the configured resolver is down or unreachable
- [ ] 5. Compare answers: different IPs from different resolvers → split-horizon, geo-DNS, or a stale static entry
- [ ] 6. Check the error type: NXDOMAIN (name really missing / search-domain appended), SERVFAIL (upstream chain broken), REFUSED (policy or recursion disabled)
- [ ] 7. For internal names: check the zone on the authoritative server and DHCP-issued search domain
- [ ] 8. Check for a captive portal / VPN DNS override hijacking port 53

#### Validation

- [ ] Internal and external names both resolve
- [ ] Negative caching cleared (previously-failing names answer)
- [ ] Latency sane (< 100ms for cached, < 500ms cold)

#### Decisions

- All resolvers fail from this host only → local path/firewall
- Fail from all hosts → resolver service or upstream connectivity
- Only internal names fail → authoritative zone or forwarding config


## No network connectivity on a host
<!-- section-tags: network, troubleshooting, connectivity -->

#### Prerequisites

- Local access to the affected host
- Know the expected: VLAN, subnet, gateway, DNS servers

#### Steps

- [ ] 1. Check the link: cable seated, NIC LED, `ip link` / adapter status — fix L1 first
- [ ] 2. Check the IP: `ip a` / `ipconfig` — right subnet? APIPA (169.254.x.x) means DHCP failed → jump to step 7
- [ ] 3. Check the gateway is reachable: `ping <gateway>` — if this fails, it is L2/VLAN or switchport, not DNS
- [ ] 4. Check ARP: `ip neigh` / `arp -a` — gateway entry present? INCOMPLETE = L2 problem
- [ ] 5. Ping beyond the gateway: `ping 8.8.8.8` — works? then L3 is fine, go to DNS
- [ ] 6. Test DNS: `dig example.com` / `nslookup example.com` — fails → fix resolver config, not routing
- [ ] 7. DHCP path: restart the client (`dhclient`, NetworkManager, `ipconfig /renew`); if still silent, check VLAN relay (`ip helper-address`) and server scope
- [ ] 8. Verify return path for the *other* side: symmetric routing, firewall state, NAT — ping from both directions

#### Validation

- [ ] Gateway ping stable (no loss over 20+ packets)
- [ ] DNS resolves a public and an internal name
- [ ] The original application works

#### Rollback

No changes made to infrastructure in this runbook; revert any host-level
config you touched (adapter disable, client restart) by rebooting the stack
or re-applying the original settings.

#### Decisions

- Gateway ping fails → work L2: VLAN membership, native VLAN, switchport, STP state
- Gateway OK, external IP fails → default route, NAT/firewall on the egress
- External IP OK, names fail → DNS: resolver config, upstream, split-horizon
