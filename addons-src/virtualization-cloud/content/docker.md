---
id = "doc/vendors/docker/docker"
kind = "vendor"
title = "Docker"
category = "reference/vendors/docker"
tags = ["docker", "compose", "cli", "logs", "troubleshooting", "networking", "containers", "model-doc"]
risk = "medium"
version = "Docker Engine 24+ / Docker Engine 24+ (Compose v2)"
---
The complete Docker document: every topic for this model in one
file (v2 document model — see docs/PACK-SPEC.md, "Document model (v2 draft)").
Content is reorganised, unchanged, from this pack's original per-command
files; each topic is a section carrying its own tags via a section-tags
marker. Source attributions are kept per topic.


## Commands
<!-- section-tags: docker, compose, cli, logs, troubleshooting, networking, containers -->


### docker compose up / ps / down

Starts a compose stack, shows its status, or tears it down.

#### Command

```bash
docker compose up -d
docker compose ps
docker compose down
```

#### Notes

- Run from the directory containing compose.yaml; `up -d` creates and starts everything detached.
- `down` stops and removes containers and networks but keeps named volumes.
- Never add `-v` to `down` unless you intend to delete the volumes (databases included).
- `docker compose logs -f <service>` tails a single service's output.

Source: Docker Engine & Compose documentation — https://docs.docker.com


### docker logs

Shows the stdout/stderr logs of a container.

#### Command

```bash
docker logs -f --tail 100 <container>
```

#### Notes

- `-f` follows the log like tail -f; `--tail 100` starts with the last 100 lines.
- `-t` adds timestamps, which helps correlate with other systems.
- Works for containers using the json-file or local log driver (the default).
- For containers that exited, logs still show the final output - ideal for crash loops.

Source: Docker Engine & Compose documentation — https://docs.docker.com


### docker network ls / inspect

Lists Docker networks and inspects one for subnets, gateways and attached containers.

#### Command

```bash
docker network ls
docker network inspect bridge
```

#### Notes

- Built-in drivers: bridge (default per-host), host, none; overlay is for Swarm.
- `inspect` shows the subnet/gateway and every container attached with its IP.
- Verify a compose service landed on the expected network here before blaming DNS.

Source: Docker Engine & Compose documentation — https://docs.docker.com


### docker ps (containers)

Lists running containers, or all containers with -a.

#### Command

```bash
docker ps
docker ps -a
```

#### Notes

- `-a` includes stopped containers - the first step when a service did not come up.
- `docker ps -q` prints only IDs, handy for scripting (`docker stop $(docker ps -q)`).
- Add `--filter name=<pattern>` or `--filter status=exited` to narrow output.
- `docker inspect <name>` gives full detail on one container.

Source: Docker Engine & Compose documentation — https://docs.docker.com

