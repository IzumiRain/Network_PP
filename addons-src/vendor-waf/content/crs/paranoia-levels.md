---
id = "ref/waf/crs-paranoia-levels"
kind = "protocol"
title = "CRS paranoia levels (PL1-PL4)"
category = "reference/security/waf/crs"
tags = ["waf", "crs", "paranoia-level"]
version = "OWASP CRS 3.x/4.x"
---
Paranoia levels trade strictness against false positives: PL1 ships as default, PL4 enables the most aggressive rule subset.

## Example

```apache
# crs-setup.conf
# setvar:tx.paranoia_level=1
```

## Notes

- PL1: baseline for most sites; PL2-3 add stricter checks (detection of unusual encodings, tighter length/character rules).
- PL4 treats even slight deviations as attacks and will break many legitimate applications - test before raising.
- Every rule is tagged with its paranoia level; rules above the configured level are skipped at runtime.
- Raise the level gradually per site and pair each bump with a review of the audit log for new false positives.

Source: OWASP Core Rule Set Documentation — https://coreruleset.org
