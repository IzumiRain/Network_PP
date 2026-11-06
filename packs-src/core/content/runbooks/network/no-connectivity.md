---
id = "rb/net/no-connectivity"
kind = "runbook"
title = "Runbook: No network connectivity on a host"
category = "runbooks/network"
tags = ["network", "troubleshooting", "connectivity"]
risk = "info"
version = "core-0.2.0"
---
## Prerequisites

- Local access to the affected host
- Know the expected: VLAN, subnet, gateway, DNS servers

## Steps

- [ ] 1. Check the link: cable seated, NIC LED, `ip link` / adapter status — fix L1 first
- [ ] 2. Check the IP: `ip a` / `ipconfig` — right subnet? APIPA (169.254.x.x) means DHCP failed → jump to step 7
- [ ] 3. Check the gateway is reachable: `ping <gateway>` — if this fails, it is L2/VLAN or switchport, not DNS
- [ ] 4. Check ARP: `ip neigh` / `arp -a` — gateway entry present? INCOMPLETE = L2 problem
- [ ] 5. Ping beyond the gateway: `ping 8.8.8.8` — works? then L3 is fine, go to DNS
- [ ] 6. Test DNS: `dig example.com` / `nslookup example.com` — fails → fix resolver config, not routing
- [ ] 7. DHCP path: restart the client (`dhclient`, NetworkManager, `ipconfig /renew`); if still silent, check VLAN relay (`ip helper-address`) and server scope
- [ ] 8. Verify return path for the *other* side: symmetric routing, firewall state, NAT — ping from both directions

## Validation

- [ ] Gateway ping stable (no loss over 20+ packets)
- [ ] DNS resolves a public and an internal name
- [ ] The original application works

## Rollback

No changes made to infrastructure in this runbook; revert any host-level
config you touched (adapter disable, client restart) by rebooting the stack
or re-applying the original settings.

## Decisions

- Gateway ping fails → work L2: VLAN membership, native VLAN, switchport, STP state
- Gateway OK, external IP fails → default route, NAT/firewall on the egress
- External IP OK, names fail → DNS: resolver config, upstream, split-horizon
