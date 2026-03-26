use async_trait::async_trait;

use crate::error::Result;
use crate::models::*;

#[async_trait]
pub trait SandboxProvider: Send + Sync {
    async fn create(&self, config: SandboxConfig) -> Result<Sandbox>;
    async fn get(&self, id: &str) -> Result<Sandbox>;
    async fn list(&self) -> Result<Vec<Sandbox>>;
    async fn start(&self, id: &str) -> Result<()>;
    async fn stop(&self, id: &str) -> Result<()>;
    async fn destroy(&self, id: &str) -> Result<()>;
}

#[async_trait]
pub trait CommandExecutor: Send + Sync {
    async fn exec(&self, sandbox_id: &str, cmd: Command) -> Result<ExecResult>;
}

#[async_trait]
pub trait FileSystem: Send + Sync {
    async fn read_file(&self, sandbox_id: &str, path: &str) -> Result<Vec<u8>>;
    async fn write_file(&self, sandbox_id: &str, path: &str, content: &[u8]) -> Result<()>;
    async fn list_dir(&self, sandbox_id: &str, path: &str) -> Result<Vec<FileEntry>>;
    async fn delete(&self, sandbox_id: &str, path: &str) -> Result<()>;
    async fn mkdir(&self, sandbox_id: &str, path: &str) -> Result<()>;
}

#[async_trait]
pub trait Snapshottable: Send + Sync {
    async fn create_snapshot(&self, sandbox_id: &str) -> Result<Snapshot>;
    async fn list_snapshots(&self, sandbox_id: &str) -> Result<Vec<Snapshot>>;
    async fn restore_snapshot(&self, sandbox_id: &str, snapshot_id: &str) -> Result<()>;
}
