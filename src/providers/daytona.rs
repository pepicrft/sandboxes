use async_trait::async_trait;
use reqwest::Client;

use crate::error::{Result, SandboxError};
use crate::models::*;
use crate::traits::*;

pub struct DaytonaProvider {
    client: Client,
    base_url: String,
    api_key: String,
}

impl DaytonaProvider {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            base_url: "https://app.daytona.io/api".into(),
            api_key: api_key.into(),
        }
    }

    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    fn auth_header(&self) -> String {
        format!("Bearer {}", self.api_key)
    }
}

#[derive(serde::Deserialize)]
struct DaytonaSandbox {
    id: String,
    state: Option<String>,
    #[serde(default)]
    metadata: std::collections::HashMap<String, String>,
}

impl From<DaytonaSandbox> for Sandbox {
    fn from(ds: DaytonaSandbox) -> Self {
        let status = match ds.state.as_deref() {
            Some("running") => SandboxStatus::Running,
            Some("stopped") => SandboxStatus::Stopped,
            Some("creating") => SandboxStatus::Creating,
            Some("starting") => SandboxStatus::Starting,
            Some("stopping") => SandboxStatus::Stopping,
            Some("error") => SandboxStatus::Failed,
            _ => SandboxStatus::Unknown,
        };
        Sandbox {
            id: ds.id,
            status,
            created_at: None,
            resources: None,
            metadata: ds.metadata,
        }
    }
}

#[derive(serde::Serialize)]
struct CreateSandboxRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<std::collections::HashMap<String, String>>,
}

#[derive(serde::Deserialize)]
struct ExecResponse {
    #[serde(default)]
    exit_code: i32,
    #[serde(default)]
    stdout: String,
    #[serde(default)]
    stderr: String,
}

#[derive(serde::Deserialize)]
struct FileEntryResponse {
    name: String,
    #[serde(rename = "type")]
    entry_type: String,
    #[serde(default)]
    size: Option<u64>,
    #[serde(default)]
    modified_at: Option<String>,
}

impl From<FileEntryResponse> for FileEntry {
    fn from(fe: FileEntryResponse) -> Self {
        FileEntry {
            name: fe.name,
            entry_type: if fe.entry_type == "directory" {
                FileEntryType::Directory
            } else {
                FileEntryType::File
            },
            size: fe.size,
            modified_at: fe.modified_at,
        }
    }
}

#[async_trait]
impl SandboxProvider for DaytonaProvider {
    async fn create(&self, config: SandboxConfig) -> Result<Sandbox> {
        let req = CreateSandboxRequest {
            image: config.image,
            env: if config.env.is_empty() {
                None
            } else {
                Some(config.env)
            },
        };

        let resp = self
            .client
            .post(self.url("/sandbox"))
            .header("Authorization", self.auth_header())
            .json(&req)
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(SandboxError::ProviderError(format!(
                "create failed: {}",
                resp.status()
            )));
        }

        let ds: DaytonaSandbox = resp.json().await?;
        Ok(ds.into())
    }

    async fn get(&self, id: &str) -> Result<Sandbox> {
        let resp = self
            .client
            .get(self.url(&format!("/sandbox/{}", id)))
            .header("Authorization", self.auth_header())
            .send()
            .await?;

        if resp.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(SandboxError::NotFound(id.to_string()));
        }

        if !resp.status().is_success() {
            return Err(SandboxError::ProviderError(format!(
                "get failed: {}",
                resp.status()
            )));
        }

        let ds: DaytonaSandbox = resp.json().await?;
        Ok(ds.into())
    }

    async fn list(&self) -> Result<Vec<Sandbox>> {
        let resp = self
            .client
            .get(self.url("/sandbox"))
            .header("Authorization", self.auth_header())
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(SandboxError::ProviderError(format!(
                "list failed: {}",
                resp.status()
            )));
        }

        let sandboxes: Vec<DaytonaSandbox> = resp.json().await?;
        Ok(sandboxes.into_iter().map(Into::into).collect())
    }

    async fn start(&self, id: &str) -> Result<()> {
        let resp = self
            .client
            .post(self.url(&format!("/sandbox/{}/start", id)))
            .header("Authorization", self.auth_header())
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(SandboxError::ProviderError(format!(
                "start failed: {}",
                resp.status()
            )));
        }
        Ok(())
    }

    async fn stop(&self, id: &str) -> Result<()> {
        let resp = self
            .client
            .post(self.url(&format!("/sandbox/{}/stop", id)))
            .header("Authorization", self.auth_header())
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(SandboxError::ProviderError(format!(
                "stop failed: {}",
                resp.status()
            )));
        }
        Ok(())
    }

    async fn destroy(&self, id: &str) -> Result<()> {
        let resp = self
            .client
            .delete(self.url(&format!("/sandbox/{}", id)))
            .header("Authorization", self.auth_header())
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(SandboxError::ProviderError(format!(
                "destroy failed: {}",
                resp.status()
            )));
        }
        Ok(())
    }
}

#[async_trait]
impl CommandExecutor for DaytonaProvider {
    async fn exec(&self, sandbox_id: &str, cmd: Command) -> Result<ExecResult> {
        let body = serde_json::json!({
            "command": cmd.args.join(" "),
            "cwd": cmd.working_dir,
            "env": if cmd.env.is_empty() { None } else { Some(&cmd.env) },
            "timeout": cmd.timeout_s,
        });

        let resp = self
            .client
            .post(self.url(&format!("/sandbox/{}/exec", sandbox_id)))
            .header("Authorization", self.auth_header())
            .json(&body)
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(SandboxError::ExecFailed(format!(
                "exec failed: {}",
                resp.status()
            )));
        }

        let exec_resp: ExecResponse = resp.json().await?;
        Ok(ExecResult {
            exit_code: exec_resp.exit_code,
            stdout: exec_resp.stdout,
            stderr: exec_resp.stderr,
            duration_ms: None,
        })
    }
}

#[async_trait]
impl FileSystem for DaytonaProvider {
    async fn read_file(&self, sandbox_id: &str, path: &str) -> Result<Vec<u8>> {
        let resp = self
            .client
            .get(self.url(&format!("/sandbox/{}/files", sandbox_id)))
            .header("Authorization", self.auth_header())
            .query(&[("path", path)])
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(SandboxError::FileError(format!(
                "read failed: {}",
                resp.status()
            )));
        }

        Ok(resp.bytes().await?.to_vec())
    }

    async fn write_file(&self, sandbox_id: &str, path: &str, content: &[u8]) -> Result<()> {
        let resp = self
            .client
            .put(self.url(&format!("/sandbox/{}/files", sandbox_id)))
            .header("Authorization", self.auth_header())
            .query(&[("path", path)])
            .body(content.to_vec())
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(SandboxError::FileError(format!(
                "write failed: {}",
                resp.status()
            )));
        }
        Ok(())
    }

    async fn list_dir(&self, sandbox_id: &str, path: &str) -> Result<Vec<FileEntry>> {
        let resp = self
            .client
            .get(self.url(&format!("/sandbox/{}/files/list", sandbox_id)))
            .header("Authorization", self.auth_header())
            .query(&[("path", path)])
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(SandboxError::FileError(format!(
                "list failed: {}",
                resp.status()
            )));
        }

        let entries: Vec<FileEntryResponse> = resp.json().await?;
        Ok(entries.into_iter().map(Into::into).collect())
    }

    async fn delete(&self, sandbox_id: &str, path: &str) -> Result<()> {
        let resp = self
            .client
            .delete(self.url(&format!("/sandbox/{}/files", sandbox_id)))
            .header("Authorization", self.auth_header())
            .query(&[("path", path)])
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(SandboxError::FileError(format!(
                "delete failed: {}",
                resp.status()
            )));
        }
        Ok(())
    }

    async fn mkdir(&self, sandbox_id: &str, path: &str) -> Result<()> {
        let resp = self
            .client
            .post(self.url(&format!("/sandbox/{}/files/mkdir", sandbox_id)))
            .header("Authorization", self.auth_header())
            .query(&[("path", path)])
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(SandboxError::FileError(format!(
                "mkdir failed: {}",
                resp.status()
            )));
        }
        Ok(())
    }
}
