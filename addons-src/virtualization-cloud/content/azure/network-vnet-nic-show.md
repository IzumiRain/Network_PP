---
id = "cmd/azure/network-vnet-nic-show"
kind = "command"
title = "az network vnet show / nic show"
category = "reference/vendors/azure"
tags = ["azure", "az-cli", "cli", "networking"]
risk = "info"
version = "Azure CLI 2.x"
---
Reads Azure virtual network and NIC details from the CLI.

## Command

```bash
az network vnet show -g <resource-group> -n <vnet-name>
az network nic show -g <resource-group> -n <nic-name>
```

## Notes

- `vnet show` returns address spaces, subnets and DNS settings; `nic show` shows private IPs, NSG and accelerated networking.
- Use `az network vnet list -g <resource-group>` first when the exact name is unknown.
- Add `-o table` or `-o tsv` for human or script-friendly output.
- Read-only and safe in production subscriptions.

Source: Azure CLI (az network) documentation — https://learn.microsoft.com
