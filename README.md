# libsandbox

> Last directory refresh: **March 27, 2026**  
> This README is a living directory of sandbox providers. If public information is missing, it should say `I don't know`.

Rust library for normalizing sandbox providers behind one interface.

## What This Repository Implements Today

- Implemented provider adapter: `Daytona`
- Planned providers tracked in this README: `Namespace`, `Modal`, `Fly.io`, `Vercel Sandbox`, `Docker`, `Runloop`, `Cloudflare`, `OpenComputer`, `Blaxel`, `OpenSandbox`, `Ona`, `GitHub Codespaces`, `Coder`, `Devin`
- Current Cargo feature flags: `daytona`

## Provider Directory

| Provider | Service model | Notable capabilities | Pricing snapshot | libsandbox status |
|---|---|---|---|---|
| [Daytona](https://www.daytona.io/) | Remote dev sandboxes / workspaces | sandbox lifecycle, exec, filesystem, snapshots, SDK + API | `I don't know` | Implemented |
| [Namespace](https://namespace.so/) | Cloud development environments | persistent environments, SSH access, session-oriented workflows | `I don't know` | Planned |
| [Modal](https://modal.com/pricing) | Serverless containers and sandboxes | containerized exec, filesystem, autoscaling, GPU workloads | Starter: `$0` + usage, includes `$30/month` credits. Team: `$250/month` + usage, includes `$100/month` credits | Planned |
| [Fly.io](https://fly.io/) | Firecracker VMs and app machines | VM-style workloads, attached volumes, process execution | Usage-based infrastructure pricing; paid plans historically layered on top of usage | Planned |
| [Vercel Sandbox](https://vercel.com/docs/vercel-sandbox) | Ephemeral microVM sandboxes | command execution, file upload/download, automation-friendly SDK | `I don't know` | Planned |
| [Docker](https://www.docker.com/) | Self-hosted containers | local or remote containers, exec, archive-based filesystem workflows | No single hosted sandbox price; cost depends on your own infrastructure and Docker licensing | Planned |
| [Runloop](https://www.runloop.ai/) | Cloud browser / VM-like agent sandboxes | low-latency command execution, browser and environment automation | `I don't know` | Planned |
| [Cloudflare](https://developers.cloudflare.com/containers/) | Container workloads on Cloudflare infrastructure | containers plus surrounding Cloudflare primitives, API-driven orchestration | Pricing depends on underlying Cloudflare products; no simple single public sandbox SKU | Planned |
| [OpenComputer](https://docs.opencomputer.dev/sdks/typescript/overview) | API-managed sandboxes | commands, filesystem, PTY, template management | `I don't know` | Planned |
| [Blaxel](https://blaxel.ai/) | AI runtime / sandbox infrastructure | process execution and API-managed environments | `I don't know` | Planned |
| [OpenSandbox](https://opensandbox.com/) | Hosted sandbox environments | command execution and file workflows | `I don't know` | Planned |
| [Ona](https://ona.com/) | Cloud engineering environments | VM-backed workspaces, tasks, Gitpod migration path | Public migration pricing example shows `Small $0.12/hr`, `Regular $0.23/hr`, `Large $0.46/hr`, `XL $1.84/hr`, `GPU $1.95/hr` | Planned |
| [GitHub Codespaces](https://docs.github.com/en/billing/managing-billing-for-your-products/managing-billing-for-github-codespaces/about-billing-for-github-codespaces) | Hosted development VMs | SSH/terminal workflows, persistent dev environments, prebuilds | Metered by machine type, storage, and transfer; pricing is usage-based rather than a flat sandbox fee | Planned |
| [Coder](https://coder.com/) | Self-hosted remote workspaces | templates, workspace lifecycle, SSH/IDE integrations | `I don't know` | Planned |
| [Devin](https://cognition.ai/) | Managed AI engineer environment | agent-driven sandbox/session workflows | `I don't know` | Planned |

## Capability Comparison

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

## Core Traits

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
- Only include publicly available information.
- If a provider does not expose public pricing, write `I don't know`.

## Source Notes

Pricing and capability summaries above were refreshed from official provider docs and pricing pages on March 27, 2026.

## License

MIT
