---
id = "cmd/checkpoint/show-asset-all"
kind = "command"
title = "show asset all (Gaia clish)"
category = "reference/vendors/check-point"
tags = ["checkpoint", "gaia", "clish", "hardware"]
risk = "info"
version = "Gaia R81.x"
---
Shows hardware inventory: platform, serial numbers, memory and disks.

## Command

```text
show asset all
```

## Notes

- Needed for RMA requests and license/host registration.
- Confirms appliance model and specs before sizing or upgrade work.
- `show asset` without `all` prints the top-level summary.

Source: Check Point Gaia R81 Security Gateway Administration Guide — https://sc1.checkpoint.com
