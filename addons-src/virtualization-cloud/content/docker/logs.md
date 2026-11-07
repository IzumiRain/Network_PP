---
id = "cmd/docker/logs"
kind = "command"
title = "docker logs"
category = "reference/vendors/docker"
tags = ["docker", "cli", "logs", "troubleshooting"]
risk = "info"
version = "Docker Engine 24+"
---
Shows the stdout/stderr logs of a container.

## Command

```bash
docker logs -f --tail 100 <container>
```

## Notes

- `-f` follows the log like tail -f; `--tail 100` starts with the last 100 lines.
- `-t` adds timestamps, which helps correlate with other systems.
- Works for containers using the json-file or local log driver (the default).
- For containers that exited, logs still show the final output - ideal for crash loops.

Source: Docker Engine & Compose documentation — https://docs.docker.com
