---
id = "err/fortigate/log-disk-full"
kind = "error"
title = "Log disk full on FortiGate"
category = "reference/vendors/fortinet/fortigate/errors"
tags = ["fortigate", "fortios", "logging"]
version = "FortiOS 7.4"
---
The FortiGate reports a disk full condition: local logging stops or starts overwriting, and the Log and Report page shows warnings.

## Common causes

- Too many log categories enabled at information or debug severity for a small flash/SSD.
- Long local retention with no log offloading, so old logs are never purged.
- A single noisy category (for example full traffic logging) filling the disk fast.

## Fix

- Reduce the enabled log categories and raise the severity threshold of noisy sources.
- Offload logs to FortiAnalyzer/SIEM instead of keeping everything on the firewall:

```cisco
config log fortianalyzer setting
    set status enable
end
```

- Delete old logs from the Log Settings page in the GUI once retention needs are met elsewhere.

Source: Fortinet FortiOS Administration Guide — https://docs.fortinet.com
