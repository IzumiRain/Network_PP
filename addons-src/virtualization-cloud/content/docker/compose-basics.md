---
id = "cmd/docker/compose-basics"
kind = "command"
title = "docker compose up / ps / down"
category = "reference/vendors/docker"
tags = ["docker", "compose", "cli"]
risk = "medium"
version = "Docker Engine 24+ (Compose v2)"
---
Starts a compose stack, shows its status, or tears it down.

## Command

```bash
docker compose up -d
docker compose ps
docker compose down
```

## Notes

- Run from the directory containing compose.yaml; `up -d` creates and starts everything detached.
- `down` stops and removes containers and networks but keeps named volumes.
- Never add `-v` to `down` unless you intend to delete the volumes (databases included).
- `docker compose logs -f <service>` tails a single service's output.

Source: Docker Engine & Compose documentation — https://docs.docker.com
