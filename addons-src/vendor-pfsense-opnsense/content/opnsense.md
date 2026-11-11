---
id = "doc/pfsense-opnsense/opnsense/opnsense"
kind = "vendor"
title = "OPNsense"
category = "reference/vendors/pfsense-opnsense/opnsense"
tags = ["opnsense", "cli", "system", "model-doc"]
risk = "info"
version = "OPNsense 24.x"
---
The complete OPNsense document: every topic for this model in one
file (v2 document model — see docs/PACK-SPEC.md, "Document model (v2 draft)").
Content is reorganised, unchanged, from this pack's original per-command
files; each topic is a section carrying its own tags via a section-tags
marker. Source attributions are kept per topic.


## Opnsense
<!-- section-tags: opnsense, cli, system -->


### opnsense-version

Prints the OPNsense product, version and build information.

#### Command

```bash
opnsense-version
```

#### Notes

- OPNsense equivalent of pfSense's `cat /etc/version`.
- Check the release before installing plugins or running upgrades.
- Combine with `cat /usr/local/etc/pkg/repos/OPNsense.conf` checks when a repo problem is suspected.

Source: OPNsense Documentation — https://docs.opnsense.org

