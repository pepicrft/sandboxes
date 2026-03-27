# Claude (Anthropic)

## Architecture

Claude Code's remote sandbox runs **Firecracker microVMs** on **KVM** (Intel Xeon Skylake hosts).

### Hypervisor

Same Firecracker build as Namespace (ACPI table date `FCAT 20240119`), but with an additional MCFG table:

```
ACPI: RSDP 0x00000000000E0000 000024 (v02 FIRECK)
ACPI: XSDT ... (v01 FIRECK FCMVXSDT 00000000 FCAT 20240119)
ACPI: FACP ... (v06 FIRECK FCVMFADT 00000000 FCAT 20240119)
ACPI: DSDT ... (v02 FIRECK FCVMDSDT 00000000 FCAT 20240119)
ACPI: APIC ... (v06 FIRECK FCVMMADT 00000000 FCAT 20240119)
ACPI: MCFG ... (v01 FIRECK FCMVMCFG 00000000 FCAT 20240119)
```

### Key differences from Namespace

| Aspect | Claude | Namespace |
|---|---|---|
| CPU | Intel Xeon Skylake | AMD EPYC Zen 4 |
| Init | `/process_api --firecracker-init` | `tini` + devbox agent |
| User | root (uid=0) | devbox (uid=1001) |
| Capabilities | Near-full | None |
| Seccomp | Disabled (mode 0) | Enabled (mode 2, 1 filter) |
| Cgroup | v1 (hybrid with v2) | v2 only |
| Networking | No interfaces, egress via HTTP proxy | Direct eth0 with gateway |
| Filesystem | Simple: 1 ext4 root + 2 squashfs | Block-level layering (9 virtio-blk) |
| Nested virt | No (/dev/kvm absent) | Yes |
| Docker | Installed but daemon not running | Full Docker with registry mirrors |
| Boot | Snapshot/restore (VM fork detected) | Cold boot (~4.5s to all mounts) |
| Write I/O | 151 MB/s | 1.1 GB/s |

### Networking model

Claude sandboxes have **no network interfaces**. All egress is routed through an HTTP proxy at `21.0.0.173:15004` using JWT-authenticated connections. The JWT tokens contain an `allowed_hosts` claim that acts as an egress allowlist, controlling which domains the sandbox can reach. Proxy env vars are set for HTTP, HTTPS, npm, yarn, and Java.

The `/etc/hosts` file maps Anthropic API endpoints and monitoring services (Sentry, Statsig, Datadog) to specific IPs.

### Snapshot/restore

The dmesg log shows `random: crng reseeded due to virtual machine fork`, indicating Claude sandboxes boot from a pre-warmed snapshot rather than cold-booting. This is consistent with the `CONFIG_USERFAULTFD=y` kernel config (enables fast VM cloning).

### Security model

More permissive inside the VM (root, full caps, no seccomp), but isolated at the network layer:
- No network interfaces at all
- Egress only through authenticated proxy with domain allowlist
- `init_on_free=1` zeroes freed memory
- `ipv6.disable=1`, `nomodule` (no kernel module loading)
- `random.trust_cpu=1` for entropy

### Egress proxy and JWT allowlist

The proxy authenticates using JWT tokens (ES256-signed, issued by `anthropic-egress-control`) passed as HTTP basic auth credentials. The JWT payload contains:

| Claim | Description |
|---|---|
| `iss` | `anthropic-egress-control` |
| `organization_uuid` | Org identifier |
| `session_id` | Claude Code session ID |
| `container_id` | Container/sandbox identifier |
| `iat` / `exp` | 4-hour validity window |
| `is_hipaa_regulated` | `false` |
| `use_egress_gateway` | `true` |
| `enforce_container_binding` | `false` |
| `allowed_hosts` | Comma-separated domain allowlist (209 domains) |

The allowlist covers public package registries (npm, PyPI, crates.io, RubyGems, Maven, etc.), source forges (GitHub, GitLab, Bitbucket), language ecosystem sites, container registries (Docker Hub, GHCR, GCR, ECR), Anthropic's own services, and monitoring (Sentry, Datadog, Statsig). No private or arbitrary domains are included.

### Inferred cloud provider

> [!NOTE]
> The following is inferred from publicly observable signals (WHOIS, CPU model, allowlist entries), not confirmed by Anthropic.

| Signal | Value | Inference |
|---|---|---|
| Sentry IP `35.186.247.156` | WHOIS: Google Cloud | GCP |
| Statsig IPs `34.36.57.103`, `34.128.128.0` | WHOIS: Google LLC | GCP |
| JWT allowlist entry `483703932474.us-east5.run.app` | GCP Cloud Run, project ID, region us-east5 | GCP (Columbus, Ohio) |
| CPU: Intel Xeon Skylake @ 2.80GHz, model 85 | Matches GCP N1/N2 instance types | GCP |
| `160.79.104.0/21` (api.anthropic.com) | WHOIS: **Anthropic, PBC** | Anthropic's own IP allocation |
| Proxy at `21.0.0.173` | WHOIS: DoD (DNIC-SNET-021) | Internal/private IP within VPC, not actually DoD |

## Specs

| Machine | vCPUs | RAM | Root disk | Spec file |
|---|---|---|---|---|
| Default | 4 | 16 GB | 256 GB | [16gb-4vcpu.md](16gb-4vcpu.md) |
