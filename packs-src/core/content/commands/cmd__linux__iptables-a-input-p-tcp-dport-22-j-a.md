---
id = "cmd/linux/iptables-a-input-p-tcp-dport-22-j-a"
kind = "command"
title = "iptables -A INPUT -p tcp --dport 22 -j ACCEPT"
category = "reference/os/linux/firewall-nftables-iptables"
tags = ["linux", "firewall-nftables-iptables"]
risk = "low"
version = "core-0.2.0"
---

**Command:** `iptables -A INPUT -p tcp --dport 22 -j ACCEPT`

Allow SSH

```bash
iptables -A INPUT -p tcp --dport 22 -j ACCEPT
```
