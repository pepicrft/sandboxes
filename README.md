# 📦 libsandbox

A universal Rust library for interacting with cloud sandbox providers. One interface, many backends.

## 🔭 Vision

Cloud sandbox providers (Daytona, Modal, Fly.io, Vercel, Docker, etc.) each expose different APIs for the same core operations: creating sandboxes, executing commands, and managing files. `libsandbox` provides a single, ergonomic Rust interface with adapters for each provider , plus language bindings for Python, TypeScript, Swift, and more.

Think of it like **libgit2 for sandboxes**, or **JDBC for cloud dev environments**.

## 🏗️ Architecture

```text
┌──────────────┐
│  Your Agent  │
└──────┬───────┘
       │  libsandbox (Rust traits)
┌──────▼───────┐
│   Adapter    │  ← one per provider
└──────┬───────┘
       │  Provider-native API
┌──────▼───────┐
│  Daytona /   │
│  Fly.io /    │
│  Modal / ... │
└──────────────┘
```

## 🌍 Supported Providers

| Provider | Isolation | Exec Model | File Ops | Snapshots | Status |
|---|---|---|---|---|---|
| Daytona | Linux VM | REST exec + stateful code | Full CRUD + find/replace | ✅ | 🚧 In progress |
| Namespace | Container (gRPC) | SSH/API exec + tmux sessions | Read/Write | ✅ (persistent volumes) | 📋 Planned |
| Modal | Container | SDK exec() | Read/Write (Alpha) | ✅ (filesystem) | 📋 Planned |
| Fly.io | Firecracker VM | REST exec (60s timeout) | Via volumes | ✅ (via suspend) | 📋 Planned |
| Vercel Sandbox | Firecracker microVM | runCommand() | Read/Write/Download | ✅ | 📋 Planned |
| Docker | Container/microVM | Exec API | Archive (tar) | ✅ (via images) | 📋 Planned |
| Runloop | VM | cmd.exec() (50ms latency) | Upload/Download | ✅ (disk) | 📋 Planned |
| Cloudflare | Durable Object + Container | exec() | Read/Write | ❌ | 📋 Planned |
| OpenComputer | KVM | commands.run() | Read/Write | ✅ (checkpoints) | 📋 Planned |
| Blaxel | microVM | Process execution | Full REST API | ❌ | 📋 Planned |
| OpenSandbox | Container/K8s | Exec commands | Read/Write | ❌ | 📋 Planned |
| Ona (ex-Gitpod) | Linux VM | Tasks API | GET files | ❌ | 📋 Planned |
| Codespaces | VM | Via SSH/PTY | Via SSH/SCP | ✅ (prebuilds) | 📋 Planned |
| Coder | Terraform-provisioned | Via SSH/PTY | Upload/Download (tar) | ✅ (templates) | 📋 Planned |
| Devin | Cloud sandbox | Via session messages | Upload attachments | ❌ | 📋 Planned |

## 🧬 Core Traits

```rust
// 🔄 Sandbox lifecycle
#[async_trait]
trait SandboxProvider {
    async fn create(&self, config: SandboxConfig) -> Result<Sandbox>;
    async fn get(&self, id: &str) -> Result<Sandbox>;
    async fn list(&self) -> Result<Vec<Sandbox>>;
    async fn start(&self, id: &str) -> Result<()>;
    async fn stop(&self, id: &str) -> Result<()>;
    async fn destroy(&self, id: &str) -> Result<()>;
}

// ⚡ Command execution
#[async_trait]
trait CommandExecutor {
    async fn exec(&self, id: &str, cmd: Command) -> Result<ExecResult>;
}

// 📁 File operations
#[async_trait]
trait FileSystem {
    async fn read_file(&self, id: &str, path: &str) -> Result<Vec<u8>>;
    async fn write_file(&self, id: &str, path: &str, content: &[u8]) -> Result<()>;
    async fn list_dir(&self, id: &str, path: &str) -> Result<Vec<FileEntry>>;
    async fn delete(&self, id: &str, path: &str) -> Result<()>;
    async fn mkdir(&self, id: &str, path: &str) -> Result<()>;
}

// 📸 Optional capability
#[async_trait]
trait Snapshottable {
    async fn create_snapshot(&self, id: &str) -> Result<Snapshot>;
    async fn list_snapshots(&self, id: &str) -> Result<Vec<Snapshot>>;
    async fn restore_snapshot(&self, id: &str, snapshot_id: &str) -> Result<()>;
}
```

## 🚀 Usage

```rust
use libsandbox::prelude::*;
use libsandbox::providers::daytona::DaytonaProvider;

#[tokio::main]
async fn main() -> libsandbox::Result<()> {
    let provider = DaytonaProvider::new("your-api-key");

    // Create a sandbox
    let sandbox = provider.create(SandboxConfig::default()).await?;

    // Execute a command
    let result = provider.exec(&sandbox.id, Command::shell("echo hello")).await?;
    println!("stdout: {}", result.stdout);

    // Write and read files
    provider.write_file(&sandbox.id, "/tmp/test.txt", b"hello world").await?;
    let content = provider.read_file(&sandbox.id, "/tmp/test.txt").await?;

    // Clean up
    provider.destroy(&sandbox.id).await?;
    Ok(())
}
```

## 📄 License

MIT
