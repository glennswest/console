# CLAUDE.md — console Project

## Overview

Unified dashboard for mkube cluster management + stormd process supervision. Single Rust binary, Dracula dark theme, inline HTML/CSS/JS (no template engine).

## Build & Deploy

```bash
# Build (macOS dev)
cargo build

# Build ARM64 + container image
./build.sh

# Build + push to local registry
./deploy.sh
```

## Architecture

- **Rust + Axum 0.8**: HTTP server with inline HTML responses
- **reqwest**: API aggregation client for mkube and stormd
- **Inline HTML/CSS/JS**: No template engine, matches stormd pattern
- **Client-side rendering**: JavaScript fetches data from mkube/stormd APIs directly
- **Multi-node aware**: Discovers nodes from mkube, queries each node's stormd

## Key Directories

```
src/main.rs          — CLI args, Axum server, routes
src/client/mkube.rs  — mkube REST API client
src/client/stormd.rs — stormd REST API client
src/ui/style.rs      — Dracula CSS + ANSI-to-HTML JS
src/ui/layout.rs     — Page shell (nav bar, head, scripts)
src/ui/dashboard.rs  — Dashboard page
src/ui/nodes.rs      — Nodes list + detail (stormd integration)
src/ui/pods.rs       — Pods list + detail + logs
src/ui/deployments.rs — Deployments list + detail
src/ui/networks.rs   — Networks list + detail (DNS/DHCP tabs)
src/ui/bmh.rs        — Bare Metal Hosts list + detail
src/ui/storage.rs    — PVCs, iSCSI CDROMs, Disks
src/ui/jobs.rs       — Jobs, runners, queue
src/ui/logs.rs       — Pod log viewer
src/ui/terminal.rs   — WebSocket terminal proxy
```

## Configuration

```
--mkube-url http://192.168.200.2:8082   # mkube API
--bind 0.0.0.0:8090                      # listen address
```

## Current Version: v0.1.0

## Work Plan

### Completed
- [x] Phase 1: Project scaffold + all UI pages
- [x] Phase 2: Pods + Deployments pages
- [x] Phase 3: Networks + DNS/DHCP detail tabs
- [x] Phase 4: BMH + Storage + Jobs pages
- [x] Phase 5: Nodes detail + stormd integration
- [x] Phase 6: Logs + Terminal pages

### TODO
- [ ] Deploy to mkube as pod, verify end-to-end
- [ ] WebSocket proxy through console (currently direct to stormd)
- [ ] BootConfigs page
- [ ] Registries page
