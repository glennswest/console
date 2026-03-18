#![allow(dead_code)]
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone)]
pub struct StormdClient {
    pub base_url: String,
    client: Client,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Process {
    pub name: Option<String>,
    pub state: Option<String>,
    pub pid: Option<i64>,
    pub exit_code: Option<i64>,
    pub restarts: Option<i64>,
    pub uptime: Option<String>,
    pub last_restart: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Stats {
    pub uptime: Option<String>,
    pub processes: Option<i64>,
    pub running: Option<i64>,
    pub total_restarts: Option<i64>,
    pub memory: Option<Value>,
    pub container_name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MemoryPoint {
    pub timestamp: Option<String>,
    pub rss: Option<u64>,
    pub vms: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Mount {
    pub device: Option<String>,
    pub mount_point: Option<String>,
    pub fs_type: Option<String>,
    pub total: Option<u64>,
    pub used: Option<u64>,
    pub available: Option<u64>,
}

impl StormdClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            client: Client::builder()
                .danger_accept_invalid_certs(true)
                .timeout(std::time::Duration::from_secs(5))
                .build()
                .unwrap(),
        }
    }

    async fn get_json<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T, String> {
        let url = format!("{}{}", self.base_url, path);
        self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("stormd request failed: {}", e))?
            .json::<T>()
            .await
            .map_err(|e| format!("stormd json parse failed: {}", e))
    }

    pub async fn list_processes(&self) -> Result<Vec<Process>, String> {
        self.get_json("/api/v1/processes").await
    }

    pub async fn stats(&self) -> Result<Stats, String> {
        self.get_json("/api/v1/stats").await
    }

    pub async fn memory_history(&self) -> Result<Vec<MemoryPoint>, String> {
        self.get_json("/api/v1/memory/history").await
    }

    pub async fn list_mounts(&self) -> Result<Vec<Mount>, String> {
        self.get_json("/api/v1/mounts").await
    }

    pub async fn restart_process(&self, name: &str) -> Result<(), String> {
        let url = format!("{}/api/v1/processes/{}/restart", self.base_url, name);
        self.client.post(&url).send().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn start_process(&self, name: &str) -> Result<(), String> {
        let url = format!("{}/api/v1/processes/{}/start", self.base_url, name);
        self.client.post(&url).send().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn stop_process(&self, name: &str) -> Result<(), String> {
        let url = format!("{}/api/v1/processes/{}/stop", self.base_url, name);
        self.client.post(&url).send().await.map_err(|e| e.to_string())?;
        Ok(())
    }
}
