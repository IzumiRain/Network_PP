---
id = "ref/splunk/outputs-conf"
kind = "configuration"
title = "outputs.conf basics (forwarding)"
category = "reference/monitoring/splunk-ops/config"
tags = ["splunk", "config", "forwarding"]
version = "Splunk Enterprise 9.x"
---
outputs.conf controls where a forwarder sends data and how it load-balances across indexers.

## Example

```ini
[tcpout]
defaultGroup = prod_indexers

[tcpout:prod_indexers]
server = idx1.example.com:9997, idx2.example.com:9997, idx3.example.com:9997
```

## Notes

- 9997 is the standard receiving (Splunk-to-Splunk) port; verify `SPLUNK_TCP` receiving is enabled on the indexers.
- Forwarders auto-load-balance across the listed servers and fail over on connection loss.
- In an indexer cluster, forwarders list the peer indexers directly or use indexer discovery instead of static lists.
- `splunk btool outputs list --debug` shows the merged, effective forwarding configuration per instance.

Source: Splunk Enterprise Admin Manual — https://help.splunk.com
