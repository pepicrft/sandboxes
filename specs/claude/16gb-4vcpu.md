---
provider: claude
machine: 16gb-4vcpu
date: 2026-03-26
---

# Claude - 16GB RAM / 4 vCPU

## System

| Property | Value |
|---|---|
| Hostname | vm |
| Kernel | 6.18.5 (SMP PREEMPT_DYNAMIC) |
| OS | Ubuntu 24.04.3 LTS |
| Architecture | x86_64 |
| Uptime at collection | 87s |

## CPU

| Property | Value |
|---|---|
| Model | Intel Xeon @ 2.80GHz (Skylake, family 6 model 85) |
| vCPUs | 4 |
| Cores per socket | 4 |
| Threads per core | 1 (no SMT) |
| Sockets | 1 |
| BogoMIPS | 5600.56 |
| Notable flags | avx512, aes, hle, rtm |

## Memory

| Property | Value |
|---|---|
| Total | 16,467 MB (~16 GB) |
| Free | 15,714 MB |
| Available | 15,927 MB |
| Swap | 0 |

## Virtualization

| Property | Value |
|---|---|
| Hypervisor | Firecracker (ACPI tables: `FIRECK`, `FCAT 20240119`) |
| KVM | Full virtualization |
| Nested virt | Not available (`/dev/kvm` not present) |
| `systemd-detect-virt` | `docker` |
| MCFG ACPI table | Present (not seen in Namespace) |

### Kernel command line

```
console=ttyS0 reboot=k panic=1 nomodule random.trust_cpu=1
ipv6.disable=1 init_on_free=1
lsm=landlock,lockdown,yama,integrity,apparmor
rdinit=/process_api --firecracker-init
```

## Boot

| Property | Value |
|---|---|
| Kernel to Freeing unused | ~31s |
| Likely snapshot restore | Yes - "random: crng reseeded due to virtual machine fork" in dmesg, uptime mismatch |
| PID 1 | `/process_api --firecracker-init --addr 0.0.0.0:2024 --max-ws-buffer-size 32768 --block-local-connections` |

## User & Permissions

| Property | Value |
|---|---|
| User | `root` (uid=0) |
| Open files limit | 4,096 |
| Max user processes | 64,294 |
| Stack size | 8,192 KB |

## Security

| Property | Value |
|---|---|
| Seccomp | Mode 0 (DISABLED, no filters) |
| Capabilities (effective) | `0x000001fffeffffff` (near-full) |
| Capabilities (bounding) | `0x000001fffeffffff` |
| LSM (cmdline) | landlock, lockdown, yama, integrity, apparmor |
| LSM (runtime) | No LSM active |
| AppArmor compiled | No (`CONFIG_SECURITY_APPARMOR is not set`) |
| Lockdown compiled | No (`CONFIG_SECURITY_LOCKDOWN_LSM is not set`) |
| Landlock compiled | No (`CONFIG_SECURITY_LANDLOCK is not set`) |
| BPF | Enabled, JIT NOT enabled (`CONFIG_BPF_JIT is not set`) |
| Userfaultfd | Yes (`CONFIG_USERFAULTFD=y`) |
| init_on_free | 1 (zeroes freed memory) |

## Isolation

| Property | Value |
|---|---|
| Cgroup version | v1 (separate controllers: cpu, cpuacct, memory, devices, freezer, blkio, pids) + unified cgroup2 mount |
| Memory limit | 9223372036854771712 (effectively unlimited, near max int64) |
| CPU limit | None |
| OOM score | 666 |
| OOM score adj | 0 |
| Overcommit | Mode 0 (heuristic), ratio 50% |

## Filesystem

### Block devices

| Device | Size | RO | Mount | Purpose |
|---|---|---|---|---|
| vda | 256 GB | No | `/` | Root filesystem (ext4, rw, 20% used, 7.1 GB) |
| vdb | 63.8 MB | Yes | `/opt/claude-code` | Claude Code (squashfs) |
| vdc | 16.8 MB | Yes | `/opt/env-runner` | Env runner (squashfs) |

### Disk layout

- No overlay filesystem, no block-level layering
- No device-mapper, no dm-verity
- No nbd devices
- Much simpler than Namespace - only 3 block devices

## I/O Performance

| Test | Result |
|---|---|
| Sequential write (256 MB) | **151 MB/s** |
| Sequential read (256 MB) | **4.3 GB/s** (cached) |

## Networking

| Property | Value |
|---|---|
| Network interfaces | None (no ip addr output) |
| DNS | 8.8.8.8, 8.8.4.4 (Google DNS) |
| Listening sockets | None |
| Egress | All traffic routed through HTTP proxy with JWT-based authentication |
| Proxy endpoint | 21.0.0.173:15004 |
| Firewall | iptables present, all chains ACCEPT (no filtering) |

### Proxy configuration

- `HTTP_PROXY`, `HTTPS_PROXY`, and runtime-specific proxy vars (npm, yarn, Java) all point to the egress gateway at `21.0.0.173:15004`
- JWT tokens embedded in proxy URLs contain an `allowed_hosts` allowlist (tokens redacted)

### /etc/hosts mappings

| Hostname | Address |
|---|---|
| api.anthropic.com | 160.79.104.10 |
| api-staging.anthropic.com | 160.79.104.10 |
| sentry.io, statsig endpoints | Various mapped addresses |

## Processes

| PID | User | Command |
|---|---|---|
| 1 | root | `/process_api --firecracker-init` (custom init, not tini) |
| - | root | `environment-manager` (runs Claude Code sessions) |
| - | root | Claude Code process (with full args) |

### Notable environment variables

| Variable | Value |
|---|---|
| `IS_SANDBOX` | yes |
| `CLAUDE_CODE_REMOTE_ENVIRONMENT_TYPE` | cloud_default |
| `CLAUDE_CODE_CONTAINER_ID` | Present |

## Runtimes & Tooling

| Tool | Path |
|---|---|
| Docker | `/usr/bin/docker` (present but daemon not running) |
| runc | `/usr/bin/runc` |
| containerd | `/usr/bin/containerd` |
| nix | Not available |
| nsc | Not available |

## Hardware

| Property | Value |
|---|---|
| GPU | None |
| PCI | No PCI devices |
| Kernel modules | Not available (`nomodule` in cmdline) |
| KVM nested virt | `/dev/kvm` not present |

## Provider Metadata

No provider metadata files found. None of the standard paths exist (`/.namespace`, `/nsc`, `/.fly`, etc.).

### Custom paths

```
/opt/claude-code/   # squashfs mount (vdb)
/opt/env-runner/    # squashfs mount (vdc)
```
