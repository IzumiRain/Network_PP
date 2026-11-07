---
id = "err/fortigate/ike-no-proposal-chosen"
kind = "error"
title = "IPsec phase 2 fails with no proposal chosen"
category = "reference/vendors/fortinet/fortigate/errors"
tags = ["fortigate", "fortios", "ipsec", "ike"]
version = "FortiOS 7.4"
---
The tunnel completes IKE phase 1 but every phase 2 negotiation fails and the event log reports no proposal chosen (IKEv2 NO_PROPOSAL_CHOSEN).

## Common causes

- Phase 2 proposal mismatch: encryption, hash or DH-group (PFS) values differ between the two peers.
- One side has PFS enabled and the other disabled.
- Phase 2 traffic selectors do not cover the actual source/destination subnets being negotiated.

## Diagnose

```cisco
diagnose vpn ike log-filter dst-addr4 203.0.113.2
diagnose debug application ike -1
diagnose debug enable
```

## Notes

- The IKE debug output shows both proposals side by side; align them (and the selectors) on both peers.
- Remember to run diagnose debug disable afterwards.

Source: Fortinet FortiOS Administration Guide — https://docs.fortinet.com
