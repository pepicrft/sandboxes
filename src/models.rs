use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct SandboxConfig {
    pub image: Option<String>,
    pub resources: Option<Resources>,
    pub env: HashMap<String, String>,
    pub timeout_s: Option<u64>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Resources {
    pub cpu: Option<u32>,
    pub memory_mb: Option<u32>,
    pub disk_mb: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct Sandbox {
    pub id: String,
    pub status: SandboxStatus,
    pub created_at: Option<String>,
    pub resources: Option<Resources>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SandboxStatus {
    Creating,
    Starting,
    Running,
    Stopping,
    Stopped,
    Failed,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Command {
    pub args: Vec<String>,
    pub working_dir: Option<String>,
    pub env: HashMap<String, String>,
    pub timeout_s: Option<u64>,
}

impl Command {
    pub fn new(args: Vec<String>) -> Self {
        Self {
            args,
            working_dir: None,
            env: HashMap::new(),
            timeout_s: None,
        }
    }

    pub fn shell(cmd: &str) -> Self {
        Self::new(vec!["sh".into(), "-c".into(), cmd.into()])
    }
}

#[derive(Debug, Clone)]
pub struct ExecResult {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub duration_ms: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub name: String,
    pub entry_type: FileEntryType,
    pub size: Option<u64>,
    pub modified_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileEntryType {
    File,
    Directory,
}

#[derive(Debug, Clone)]
pub struct Snapshot {
    pub id: String,
    pub created_at: Option<String>,
    pub name: Option<String>,
}
