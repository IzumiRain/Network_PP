---
id = "doc/sophos/sophos/sophos"
kind = "vendor"
title = "Sophos Firewall"
category = "reference/vendors/sophos"
tags = ["sophos", "sfos", "cli", "shell", "routing", "model-doc"]
risk = "high"
version = "Sophos Firewall OS 19.x+"
---
The complete Sophos Firewall document: every topic for this model in one
file (v2 document model — see docs/PACK-SPEC.md, "Document model (v2 draft)").
Content is reorganised, unchanged, from this pack's original per-command
files; each topic is a section carrying its own tags via a section-tags
marker. Source attributions are kept per topic.


## Commands
<!-- section-tags: sophos, sfos, cli, shell, routing -->


### advanced shell (SFOS)

Enters the Linux advanced shell from the SFOS console for low-level troubleshooting.

#### Command

```text
sudo
```

#### Notes

- Drops into a Linux shell used only for vendor-documented procedures and support-driven steps.
- Changes made here can break the appliance and are typically unsupported - document what you do.
- The exact access method (sudo with the admin password, or support-enabled accounts) varies by SFOS release - verify in the current CLI guide.
- Leave the shell promptly and confirm normal services afterwards.

Source: Sophos Firewall CLI Guide — https://docs.sophos.com


### SSH console access (SFOS)

Opens the Sophos Firewall CLI console over SSH with an administrator account.

#### Command

```bash
ssh admin@<firewall-ip>
```

#### Notes

- The SSH console is the primary CLI; log in on TCP 22 with an admin user.
- The prompt accepts SFOS CLI commands (system, network, and related families).
- SSH access per zone is controlled under Administration > Device access.
- Some tasks are WebAdmin-only; the CLI is intentionally limited.

Source: Sophos Firewall CLI Guide — https://docs.sophos.com


### system route print (SFOS)

Displays the SFOS routing table (connected, static and dynamic routes).

#### Command

```text
system route print
```

#### Notes

- Verify expected static routes here before debugging VPN or WAN failover issues.
- Subcommands in the `system route` family vary between SFOS releases - confirm the exact syntax in the CLI guide for your firmware version.
- The WebAdmin routing view shows the same data in table form.

Source: Sophos Firewall CLI Guide — https://docs.sophos.com

