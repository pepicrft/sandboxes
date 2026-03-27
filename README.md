# libsandbox

> Last directory refresh: **March 27, 2026**  
> This README is intended to be a living directory of sandbox providers and should be updated whenever a provider is added, removed, or materially changes pricing/capabilities.

`libsandbox` is a Rust library for normalizing sandbox providers behind one interface. The goal is simple: one trait-based API for creating sandboxes, executing commands, managing files, and, where possible, working with snapshots.

## What This Repository Implements Today

- Implemented provider adapter: `Daytona`
- Planned providers tracked in this README: `Namespace`, `Modal`, `Fly.io`, `Vercel Sandbox`, `Docker`, `Runloop`, `Cloudflare`, `OpenComputer`, `Blaxel`, `OpenSandbox`, `Ona`, `GitHub Codespaces`, `Coder`, `Devin`
- Current Cargo feature flags: `daytona`

## Why This Exists

Most sandbox products expose the same primitives through different APIs:

- create and destroy an environment
- execute commands
- read and write files
- manage longer-lived state or snapshots

`libsandbox` provides a common abstraction so agents and tools can swap providers without rewriting everything around each vendor API.

## Provider Directory

| Provider | Service model | Notable capabilities | Pricing snapshot | libsandbox status |
|---|---|---|---|---|
| [Daytona](https://www.daytona.io/) | Remote dev sandboxes / workspaces | sandbox lifecycle, exec, filesystem, snapshots, SDK + API | Pricing not clearly published on a public self-serve pricing page | Implemented |
| [Namespace](https://namespace.so/) | Cloud development environments | persistent environments, SSH access, session-oriented workflows | Pricing not clearly published on a public self-serve pricing page | Planned |
| [Modal](https://modal.com/pricing) | Serverless containers and sandboxes | containerized exec, filesystem, autoscaling, GPU workloads | Starter: `$0` + usage, includes `$30/month` credits. Team: `$250/month` + usage, includes `$100/month` credits | Planned |
| [Fly.io](https://fly.io/) | Firecracker VMs and app machines | VM-style workloads, attached volumes, process execution | Usage-based infrastructure pricing; paid plans historically layered on top of usage | Planned |
| [Vercel Sandbox](https://vercel.com/docs/vercel-sandbox) | Ephemeral microVM sandboxes | command execution, file upload/download, automation-friendly SDK | Pricing not clearly published as a standalone public line item | Planned |
| [Docker](https://www.docker.com/) | Self-hosted containers | local or remote containers, exec, archive-based filesystem workflows | No single hosted sandbox price; cost depends on your own infrastructure and Docker licensing | Planned |
| [Runloop](https://www.runloop.ai/) | Cloud browser / VM-like agent sandboxes | low-latency command execution, browser and environment automation | Pricing not clearly published on a public self-serve pricing page | Planned |
| [Cloudflare](https://developers.cloudflare.com/containers/) | Container workloads on Cloudflare infrastructure | containers plus surrounding Cloudflare primitives, API-driven orchestration | Pricing depends on underlying Cloudflare products; no simple single public sandbox SKU | Planned |
| [OpenComputer](https://docs.opencomputer.dev/sdks/typescript/overview) | API-managed sandboxes | commands, filesystem, PTY, template management | Pricing not clearly published on a public self-serve pricing page | Planned |
| [Blaxel](https://blaxel.ai/) | AI runtime / sandbox infrastructure | process execution and API-managed environments | Pricing not clearly published on a public self-serve pricing page | Planned |
| [OpenSandbox](https://opensandbox.com/) | Hosted sandbox environments | command execution and file workflows | Pricing not clearly published on a public self-serve pricing page | Planned |
| [Ona](https://ona.com/) | Cloud engineering environments | VM-backed workspaces, tasks, Gitpod migration path | Public migration pricing example shows `Small $0.12/hr`, `Regular $0.23/hr`, `Large $0.46/hr`, `XL $1.84/hr`, `GPU $1.95/hr` | Planned |
| [GitHub Codespaces](https://docs.github.com/en/billing/managing-billing-for-your-products/managing-billing-for-github-codespaces/about-billing-for-github-codespaces) | Hosted development VMs | SSH/terminal workflows, persistent dev environments, prebuilds | Metered by machine type, storage, and transfer; pricing is usage-based rather than a flat sandbox fee | Planned |
| [Coder](https://coder.com/) | Self-hosted remote workspaces | templates, workspace lifecycle, SSH/IDE integrations | Public enterprise pricing is not clearly exposed as a simple self-serve sandbox price | Planned |
| [Devin](https://cognition.ai/) | Managed AI engineer environment | agent-driven sandbox/session workflows | Pricing not clearly published on a public self-serve pricing page | Planned |

## Capability Comparison

This table is intentionally coarse-grained. It is meant to help contributors decide whether a provider can plausibly implement the `libsandbox` traits, not to document every vendor-specific API detail.

| Provider | Isolation model | Command execution | File operations | Persistence / volumes | Snapshot-like workflow |
|---|---|---|---|---|---|
| Daytona | VM-style sandbox | Yes | Yes | Yes | Yes |
| Namespace | Remote dev environment | Yes | Yes | Yes | Partial / provider-specific |
| Modal | Container sandbox | Yes | Yes | Yes | Partial / filesystem-oriented |
| Fly.io | Firecracker VM | Yes | Partial / volume-centric | Yes | Partial / suspend or image-based workflows |
| Vercel Sandbox | microVM | Yes | Yes | Partial | Partial |
| Docker | Container | Yes | Yes | Yes | Partial / image-based |
| Runloop | Managed sandbox / VM | Yes | Yes | Yes | Provider-specific |
| Cloudflare | Container platform | Yes | Yes | Partial | No clear generic snapshot primitive |
| OpenComputer | Managed sandbox | Yes | Yes | Yes | Provider-specific |
| Blaxel | Managed runtime | Yes | Partial | Partial | No clear generic snapshot primitive |
| OpenSandbox | Hosted sandbox | Yes | Yes | Partial | No clear generic snapshot primitive |
| Ona | VM workspace | Yes | Partial | Yes | Partial |
| GitHub Codespaces | VM workspace | Yes | Yes | Yes | Partial / prebuilds rather than snapshots |
| Coder | Workspace platform | Yes | Yes | Yes | Partial / template-oriented |
| Devin | Managed session sandbox | Indirect / agent-mediated | Limited | Partial | No clear generic snapshot primitive |

## Commercial Comparison

| Provider | Self-serve pricing clarity | Pricing shape | Practical note for `libsandbox` |
|---|---|---|---|
| Daytona | Low | Unknown / sales-led | Good candidate technically, harder to compare commercially |
| Namespace | Low | Unknown / sales-led | Similar issue: capability signal is easier to find than price signal |
| Modal | High | Subscription + metered compute | Easiest benchmark for public pricing comparisons |
| Fly.io | Medium | Usage-based infra billing | Good fit when users want VM semantics and infra-style pricing |
| Vercel Sandbox | Low | Bundled / unclear standalone SKU | Better treated as platform capability than a clean pricing row |
| Docker | Medium | Infra-dependent | Useful baseline for self-hosted adapters, not apples-to-apples SaaS pricing |
| Runloop | Low | Unknown / likely sales-led | Strong sandbox focus, weaker public commercial transparency |
| Cloudflare | Medium | Product-composed pricing | Integration may require combining multiple Cloudflare billing dimensions |
| OpenComputer | Low | Unknown / sales-led | API surface looks promising, commercial model is less explicit |
| Blaxel | Low | Unknown / sales-led | Similar to newer agent-infra vendors |
| OpenSandbox | Low | Unknown / sales-led | Public pricing visibility appears limited |
| Ona | Medium | Hourly machine pricing | One of the more usable public benchmark points for workspace-style sandboxes |
| GitHub Codespaces | High | Metered usage | Strong public documentation, easy for contributors to keep current |
| Coder | Low | Enterprise / custom | More relevant for self-hosted enterprise adapters than public SaaS comparison |
| Devin | Low | Unknown / product-bundled | Better treated as an adjacent managed environment than a clean sandbox API vendor |

## Repository Architecture

- [src/lib.rs](/Users/pepicrft/src/github.com/pepicrft/sandboxes/src/lib.rs)
- [src/models.rs](/Users/pepicrft/src/github.com/pepicrft/sandboxes/src/models.rs)
- [src/traits.rs](/Users/pepicrft/src/github.com/pepicrft/sandboxes/src/traits.rs)
- [src/error.rs](/Users/pepicrft/src/github.com/pepicrft/sandboxes/src/error.rs)
- [src/providers/daytona.rs](/Users/pepicrft/src/github.com/pepicrft/sandboxes/src/providers/daytona.rs)

Core trait shape:

```rust
#[async_trait]
trait SandboxProvider {
    async fn create(&self, config: SandboxConfig) -> Result<Sandbox>;
    async fn get(&self, id: &str) -> Result<Sandbox>;
    async fn list(&self) -> Result<Vec<Sandbox>>;
    async fn start(&self, id: &str) -> Result<()>;
    async fn stop(&self, id: &str) -> Result<()>;
    async fn destroy(&self, id: &str) -> Result<()>;
}

#[async_trait]
trait CommandExecutor {
    async fn exec(&self, id: &str, cmd: Command) -> Result<ExecResult>;
}

#[async_trait]
trait FileSystem {
    async fn read_file(&self, id: &str, path: &str) -> Result<Vec<u8>>;
    async fn write_file(&self, id: &str, path: &str, content: &[u8]) -> Result<()>;
    async fn list_dir(&self, id: &str, path: &str) -> Result<Vec<FileEntry>>;
    async fn delete(&self, id: &str, path: &str) -> Result<()>;
    async fn mkdir(&self, id: &str, path: &str) -> Result<()>;
}

#[async_trait]
trait Snapshottable {
    async fn create_snapshot(&self, id: &str) -> Result<Snapshot>;
    async fn list_snapshots(&self, id: &str) -> Result<Vec<Snapshot>>;
    async fn restore_snapshot(&self, id: &str, snapshot_id: &str) -> Result<()>;
}
```

## Usage

```rust
use libsandbox::{Command, CommandExecutor, FileSystem, SandboxConfig, SandboxProvider};
use libsandbox::providers::daytona::DaytonaProvider;

#[tokio::main]
async fn main() -> libsandbox::Result<()> {
    let provider = DaytonaProvider::new("your-api-key");
    let sandbox = provider.create(SandboxConfig::default()).await?;

    let result = provider.exec(&sandbox.id, Command::shell("echo hello")).await?;
    println!("stdout: {}", result.stdout);

    provider
        .write_file(&sandbox.id, "/tmp/test.txt", b"hello world")
        .await?;

    let content = provider.read_file(&sandbox.id, "/tmp/test.txt").await?;
    println!("bytes: {}", content.len());

    provider.destroy(&sandbox.id).await?;
    Ok(())
}
```

Build with:

```bash
cargo build --features daytona
```

## Maintenance Notes

- When adding a new adapter, update both the implementation status and the provider directory above.
- Prefer official vendor pricing or billing documentation over blog posts or third-party comparisons.
- If a provider does not expose public pricing, say that explicitly instead of guessing.
- If a provider only partially maps to `SandboxProvider`, `CommandExecutor`, `FileSystem`, or `Snapshottable`, call that out in the capability table.

## Source Notes

Pricing and capability summaries above were refreshed from official provider docs and pricing pages on March 27, 2026. These details can change quickly and should be re-checked during README updates.

## License

MIT
