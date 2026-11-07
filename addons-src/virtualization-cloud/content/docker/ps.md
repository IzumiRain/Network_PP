---
id = "cmd/docker/ps"
kind = "command"
title = "docker ps (containers)"
category = "reference/virtualization/docker"
tags = ["docker", "cli", "containers"]
risk = "info"
version = "Docker Engine 24+"
---
Lists running containers, or all containers with -a.

## Command

```bash
docker ps
docker ps -a
```

## Notes

- `-a` includes stopped containers - the first step when a service did not come up.
- `docker ps -q` prints only IDs, handy for scripting (`docker stop $(docker ps -q)`).
- Add `--filter name=<pattern>` or `--filter status=exited` to narrow output.
- `docker inspect <name>` gives full detail on one container.

Source: Docker Engine & Compose documentation — https://docs.docker.com
