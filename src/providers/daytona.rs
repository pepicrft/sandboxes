use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::error::{Result, SandboxError};
use crate::models::*;
use crate::traits::*;

pub struct DaytonaProvider {
    client: Client,
    base_url: String,
    api_key: String,
    organization_id: Option<String>,
}

impl DaytonaProvider {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            base_url: "https://app.daytona.io/api".into(),
            api_key: api_key.into(),
            organization_id: None,
        }
    }

    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    pub fn with_organization_id(mut self, org_id: impl Into<String>) -> Self {
        self.organization_id = Some(org_id.into());
        self
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    fn auth_header(&self) -> String {
        format!("Bearer {}", self.api_key)
    }

    fn add_headers(&self, req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        let req = req.header("Authorization", self.auth_header());
        if let Some(org_id) = &self.organization_id {
            req.header("X-Daytona-Organization-ID", org_id)
        } else {
            req
        }
    }

    async fn get_toolbox_url(&self, sandbox_id: &str) -> Result<String> {
        let sandbox = self.get(sandbox_id).await?;
        sandbox
            .metadata
            .get("toolbox_proxy_url")
            .cloned()
            .ok_or_else(|| {
                SandboxError::ProviderError(format!(
                    "sandbox {} has no toolbox_proxy_url (is it running?)",
                    sandbox_id
                ))
            })
    }
}

// -- Daytona API response models --

#[derive(Deserialize)]
struct DaytonaSandbox {
    id: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    state: Option<String>,
    #[serde(default)]
    snapshot: Option<String>,
    #[serde(default)]
    user: Option<String>,
    #[serde(default)]
    target: Option<String>,
    #[serde(default)]
    cpu: Option<f64>,
    #[serde(default)]
    memory: Option<f64>,
    #[serde(default)]
    disk: Option<f64>,
    #[serde(default)]
    created_at: Option<String>,
    #[serde(default)]
    toolbox_proxy_url: Option<String>,
    #[serde(default)]
    labels: HashMap<String, String>,
    #[serde(default)]
    #[allow(dead_code)]
    env: HashMap<String, String>,
}

impl From<DaytonaSandbox> for Sandbox {
    fn from(ds: DaytonaSandbox) -> Self {
        let status = match ds.state.as_deref() {
            Some("started") => SandboxStatus::Running,
            Some("stopped") => SandboxStatus::Stopped,
            Some("creating") => SandboxStatus::Creating,
            Some("starting") => SandboxStatus::Starting,
            Some("stopping") => SandboxStatus::Stopping,
            Some("error") | Some("build_failed") => SandboxStatus::Failed,
            _ => SandboxStatus::Unknown,
        };

        let mut metadata = ds.labels;
        if let Some(name) = Some(ds.name) {
            metadata.insert("name".into(), name);
        }
        if let Some(snapshot) = ds.snapshot {
            metadata.insert("snapshot".into(), snapshot);
        }
        if let Some(user) = ds.user {
            metadata.insert("user".into(), user);
        }
        if let Some(target) = ds.target {
            metadata.insert("target".into(), target);
        }
        if let Some(url) = ds.toolbox_proxy_url {
            metadata.insert("toolbox_proxy_url".into(), url);
        }

        let resources = Some(Resources {
            cpu: ds.cpu.map(|c| c as u32),
            memory_mb: ds.memory.map(|m| (m * 1024.0) as u32),
            disk_mb: ds.disk.map(|d| (d * 1024.0) as u32),
        });

        Sandbox {
            id: ds.id,
            status,
            created_at: ds.created_at,
            resources,
            metadata,
        }
    }
}

#[derive(Serialize)]
struct CreateSandboxRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    snapshot: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_stop_interval: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<String>,
}

// -- Toolbox API models --

#[derive(Serialize)]
struct ExecRequest {
    command: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    cwd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<u64>,
}

#[derive(Deserialize)]
struct ExecResponse {
    #[serde(default)]
    exit_code: Option<i32>,
    #[serde(default)]
    result: String,
}

#[derive(Deserialize)]
struct DaytonaFileInfo {
    name: String,
    #[serde(default)]
    is_dir: bool,
    #[serde(default)]
    size: i64,
    #[serde(default)]
    mod_time: Option<String>,
}

impl From<DaytonaFileInfo> for FileEntry {
    fn from(fi: DaytonaFileInfo) -> Self {
        FileEntry {
            name: fi.name,
            entry_type: if fi.is_dir {
                FileEntryType::Directory
            } else {
                FileEntryType::File
            },
            size: if fi.size >= 0 {
                Some(fi.size as u64)
            } else {
                None
            },
            modified_at: fi.mod_time,
        }
    }
}

// -- Trait implementations --

#[async_trait]
impl SandboxProvider for DaytonaProvider {
    async fn create(&self, config: SandboxConfig) -> Result<Sandbox> {
        let resources = config.resources.as_ref();
        let req = CreateSandboxRequest {
            snapshot: config.image.clone(),
            name: config.metadata.get("name").cloned(),
            user: config.metadata.get("user").cloned(),
            env: if config.env.is_empty() {
                None
            } else {
                Some(config.env)
            },
            labels: if config.metadata.is_empty() {
                None
            } else {
                Some(config.metadata)
            },
            cpu: resources.and_then(|r| r.cpu),
            memory: resources.and_then(|r| r.memory_mb.map(|m| m / 1024)),
            disk: resources.and_then(|r| r.disk_mb.map(|d| d / 1024)),
            auto_stop_interval: config.timeout_s,
            target: None,
        };

        let resp = self
            .add_headers(self.client.post(self.url("/sandbox")))
            .json(&req)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(SandboxError::ProviderError(format!(
                "create failed ({}): {}",
                status, body
            )));
        }

        let ds: DaytonaSandbox = resp.json().await?;
        Ok(ds.into())
    }

    async fn get(&self, id: &str) -> Result<Sandbox> {
        let resp = self
            .add_headers(self.client.get(self.url(&format!("/sandbox/{}", id))))
            .send()
            .await?;

        if resp.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(SandboxError::NotFound(id.to_string()));
        }

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(SandboxError::ProviderError(format!(
                "get failed ({}): {}",
                status, body
            )));
        }

        let ds: DaytonaSandbox = resp.json().await?;
        Ok(ds.into())
    }

    async fn list(&self) -> Result<Vec<Sandbox>> {
        let resp = self
            .add_headers(self.client.get(self.url("/sandbox")))
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(SandboxError::ProviderError(format!(
                "list failed ({}): {}",
                status, body
            )));
        }

        let sandboxes: Vec<DaytonaSandbox> = resp.json().await?;
        Ok(sandboxes.into_iter().map(Into::into).collect())
    }

    async fn start(&self, id: &str) -> Result<()> {
        let resp = self
            .add_headers(self.client.post(self.url(&format!("/sandbox/{}/start", id))))
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(SandboxError::ProviderError(format!(
                "start failed ({}): {}",
                status, body
            )));
        }
        Ok(())
    }

    async fn stop(&self, id: &str) -> Result<()> {
        let resp = self
            .add_headers(self.client.post(self.url(&format!("/sandbox/{}/stop", id))))
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(SandboxError::ProviderError(format!(
                "stop failed ({}): {}",
                status, body
            )));
        }
        Ok(())
    }

    async fn destroy(&self, id: &str) -> Result<()> {
        let resp = self
            .add_headers(self.client.delete(self.url(&format!("/sandbox/{}", id))))
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(SandboxError::ProviderError(format!(
                "destroy failed ({}): {}",
                status, body
            )));
        }
        Ok(())
    }
}

#[async_trait]
impl CommandExecutor for DaytonaProvider {
    async fn exec(&self, sandbox_id: &str, cmd: Command) -> Result<ExecResult> {
        let toolbox_url = self.get_toolbox_url(sandbox_id).await?;

        let req = ExecRequest {
            command: cmd.args.join(" "),
            cwd: cmd.working_dir,
            timeout: cmd.timeout_s,
        };

        let resp = self
            .add_headers(
                self.client
                    .post(format!("{}/process/execute", toolbox_url)),
            )
            .json(&req)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(SandboxError::ExecFailed(format!(
                "exec failed ({}): {}",
                status, body
            )));
        }

        let exec_resp: ExecResponse = resp.json().await?;
        Ok(ExecResult {
            exit_code: exec_resp.exit_code.unwrap_or(-1),
            stdout: exec_resp.result,
            stderr: String::new(),
            duration_ms: None,
        })
    }
}

#[async_trait]
impl FileSystem for DaytonaProvider {
    async fn read_file(&self, sandbox_id: &str, path: &str) -> Result<Vec<u8>> {
        let toolbox_url = self.get_toolbox_url(sandbox_id).await?;

        let resp = self
            .add_headers(
                self.client
                    .get(format!("{}/files/download", toolbox_url)),
            )
            .query(&[("path", path)])
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(SandboxError::FileError(format!(
                "read failed ({}): {}",
                status, body
            )));
        }

        Ok(resp.bytes().await?.to_vec())
    }

    async fn write_file(&self, sandbox_id: &str, path: &str, content: &[u8]) -> Result<()> {
        let toolbox_url = self.get_toolbox_url(sandbox_id).await?;

        let part = reqwest::multipart::Part::bytes(content.to_vec()).file_name("file");
        let form = reqwest::multipart::Form::new().part("file", part);

        let resp = self
            .add_headers(
                self.client
                    .post(format!("{}/files/upload", toolbox_url)),
            )
            .query(&[("path", path)])
            .multipart(form)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(SandboxError::FileError(format!(
                "write failed ({}): {}",
                status, body
            )));
        }
        Ok(())
    }

    async fn list_dir(&self, sandbox_id: &str, path: &str) -> Result<Vec<FileEntry>> {
        let toolbox_url = self.get_toolbox_url(sandbox_id).await?;

        let resp = self
            .add_headers(
                self.client
                    .get(format!("{}/files", toolbox_url)),
            )
            .query(&[("path", path)])
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(SandboxError::FileError(format!(
                "list failed ({}): {}",
                status, body
            )));
        }

        let entries: Vec<DaytonaFileInfo> = resp.json().await?;
        Ok(entries.into_iter().map(Into::into).collect())
    }

    async fn delete(&self, sandbox_id: &str, path: &str) -> Result<()> {
        let toolbox_url = self.get_toolbox_url(sandbox_id).await?;

        let resp = self
            .add_headers(
                self.client
                    .delete(format!("{}/files", toolbox_url)),
            )
            .query(&[("path", path)])
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(SandboxError::FileError(format!(
                "delete failed ({}): {}",
                status, body
            )));
        }
        Ok(())
    }

    async fn mkdir(&self, sandbox_id: &str, path: &str) -> Result<()> {
        let toolbox_url = self.get_toolbox_url(sandbox_id).await?;

        let resp = self
            .add_headers(
                self.client
                    .post(format!("{}/files/folder", toolbox_url)),
            )
            .query(&[("path", path), ("mode", "0755")])
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(SandboxError::FileError(format!(
                "mkdir failed ({}): {}",
                status, body
            )));
        }
        Ok(())
    }
}
