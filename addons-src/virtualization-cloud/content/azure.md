---
id = "doc/vendors/azure/azure"
kind = "vendor"
title = "Microsoft Azure"
category = "reference/vendors/azure"
tags = ["azure", "az-cli", "cli", "networking", "model-doc"]
risk = "info"
version = "Azure CLI 2.x"
---
The complete Microsoft Azure document: every topic for this model in one
file (v2 document model — see docs/PACK-SPEC.md, "Document model (v2 draft)").
Content is reorganised, unchanged, from this pack's original per-command
files; each topic is a section carrying its own tags via a section-tags
marker. Source attributions are kept per topic.


## Commands
<!-- section-tags: azure, az-cli, cli, networking -->


### az network vnet show / nic show

Reads Azure virtual network and NIC details from the CLI.

#### Command

```bash
az network vnet show -g <resource-group> -n <vnet-name>
az network nic show -g <resource-group> -n <nic-name>
```

#### Notes

- `vnet show` returns address spaces, subnets and DNS settings; `nic show` shows private IPs, NSG and accelerated networking.
- Use `az network vnet list -g <resource-group>` first when the exact name is unknown.
- Add `-o table` or `-o tsv` for human or script-friendly output.
- Read-only and safe in production subscriptions.

Source: Azure CLI (az network) documentation — https://learn.microsoft.com

