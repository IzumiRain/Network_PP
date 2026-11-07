---
id = "ref/splunk/monitoring-console"
kind = "configuration"
title = "Monitoring Console (DMC)"
category = "reference/monitoring/splunk-ops/monitoring"
tags = ["splunk", "monitoring", "operations"]
version = "Splunk Enterprise 9.x"
---
The Splunk Monitoring Console is the built-in health dashboard for instance performance, queues and forwarder status.

## Notes

- A standalone instance monitors itself by default; distributed deployments must switch the console to distributed mode and add instances.
- Monitoring targets include search activity, indexing performance, queue status (index queue, parsing) and forwarder connectivity.
- The console reads metrics via REST and _internal data, so the instance running it needs management access (8089) to every monitored node.
- First checks for "Splunk is slow" complaints: Monitoring Console resource usage and index/queue dashboards before diving into splunkd.log.
- Enable the forwarder monitoring dashboard to spot forwarders that stopped phoning home.

Source: Splunk Enterprise Admin Manual — https://help.splunk.com
