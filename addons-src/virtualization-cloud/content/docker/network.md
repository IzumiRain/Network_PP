---
id = "cmd/docker/network"
kind = "command"
title = "docker network ls / inspect"
category = "reference/virtualization/docker"
tags = ["docker", "cli", "networking"]
risk = "info"
version = "Docker Engine 24+"
---
Lists Docker networks and inspects one for subnets, gateways and attached containers.

## Command

```bash
docker network ls
docker network inspect bridge
```

## Notes

- Built-in drivers: bridge (default per-host), host, none; overlay is for Swarm.
- `inspect` shows the subnet/gateway and every container attached with its IP.
- Verify a compose service landed on the expected network here before blaming DNS.

Source: Docker Engine & Compose documentation — https://docs.docker.com
