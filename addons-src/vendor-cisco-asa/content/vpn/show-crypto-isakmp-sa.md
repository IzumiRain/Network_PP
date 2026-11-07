---
id = "cmd/asa/show-crypto-isakmp-sa"
kind = "command"
title = "show crypto isakmp sa"
category = "reference/firewalls/cisco-asa/vpn"
tags = ["asa", "cisco", "cli", "vpn", "ike"]
risk = "info"
version = "ASA 9.16"
---
Shows IKE (phase 1) security-association state with peers and negotiation status.

## Command

```cisco
show crypto isakmp sa
show crypto isakmp sa detail
```

## Notes

- `MM_ACTIVE` means phase 1 is up; states stuck in MM_NO_STATE / MM_KEY_EXCH usually mean ISAKMP policy or UDP 500/4500 mismatch.
- `MM_WAIT_MSG_6`-style hangs often point to authentication (PSK/cert) or proposal failures.
- IKEv2 sessions appear in the same output; `detail` adds local/remote identities and durations.
- Check `show run crypto isakmp` (or `ikev2`) for the policy and `debug crypto ikev1/ikev2 <peer>` when SAs never form.

Source: Cisco ASA Series Command Reference — https://www.cisco.com
