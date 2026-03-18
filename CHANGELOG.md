# Changelog

## [Unreleased]

### 2026-03-18
- **feat:** BootConfigs page — list + detail with kernel/initrd/iPXE/cmdline fields, referencing BMH cross-link
- **feat:** Registries page — list + detail with URL/mirrors/insecure/images, delete support
- **feat:** mkube client: bootconfigs and registries API endpoints (list, get, delete)
- **feat:** Navigation bar updated with BootConfigs and Registries links

## [v0.1.0] — 2026-03-18

### Added
- Project scaffold: Cargo.toml, Dockerfile (scratch), build.sh, deploy.sh
- Dracula dark theme CSS matching stormd's UI (style.rs)
- Shared layout with nav bar across all pages (layout.rs)
- mkube REST API client covering all endpoints (client/mkube.rs)
- stormd REST API client for process management (client/stormd.rs)
- Dashboard page: cluster overview stats, node cards, events, consistency
- Nodes page: list + detail with stormd process management, mounts, pods
- Pods page: list with filters + detail with logs, labels, annotations
- Deployments page: list + detail with owned pods
- Networks page: list + detail with DNS records, DHCP pools/reservations/leases, forwarders, smoketest
- BMH page: list + detail with power on/off toggle
- Storage page: PVCs, iSCSI CDROMs, iSCSI Disks with capacity
- Jobs page: jobs list, runners, queue, job logs viewer
- Logs page: pod log viewer with node/pod filters
- Terminal page: WebSocket terminal proxy to stormd
- ANSI-to-HTML converter (JavaScript)
- Status badge system with Dracula colors
- Health endpoint at /healthz
- Auto-refresh on all list pages (15-30s intervals)
