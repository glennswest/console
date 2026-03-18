#![allow(dead_code)]
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone)]
pub struct MkubeClient {
    pub base_url: String,
    client: Client,
}

// --- Response types ---

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HealthResponse {
    pub status: Option<String>,
    pub version: Option<String>,
    pub commit: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Node {
    pub metadata: Option<Metadata>,
    pub status: Option<NodeStatus>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NodeStatus {
    pub addresses: Option<Vec<NodeAddress>>,
    #[serde(rename = "nodeInfo")]
    pub node_info: Option<NodeInfo>,
    pub conditions: Option<Vec<NodeCondition>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NodeAddress {
    #[serde(rename = "type")]
    pub addr_type: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NodeInfo {
    pub architecture: Option<String>,
    #[serde(rename = "operatingSystem")]
    pub operating_system: Option<String>,
    #[serde(rename = "kubeletVersion")]
    pub kubelet_version: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NodeCondition {
    #[serde(rename = "type")]
    pub cond_type: Option<String>,
    pub status: Option<String>,
    #[serde(rename = "lastHeartbeatTime")]
    pub last_heartbeat: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Metadata {
    pub name: Option<String>,
    pub namespace: Option<String>,
    pub labels: Option<Value>,
    pub annotations: Option<Value>,
    #[serde(rename = "creationTimestamp")]
    pub creation_timestamp: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Pod {
    pub metadata: Option<Metadata>,
    pub spec: Option<PodSpec>,
    pub status: Option<PodStatus>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PodSpec {
    pub containers: Option<Vec<Container>>,
    pub volumes: Option<Vec<Value>>,
    #[serde(rename = "nodeName")]
    pub node_name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Container {
    pub name: Option<String>,
    pub image: Option<String>,
    pub ports: Option<Vec<Value>>,
    #[serde(rename = "volumeMounts")]
    pub volume_mounts: Option<Vec<Value>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PodStatus {
    pub phase: Option<String>,
    #[serde(rename = "podIP")]
    pub pod_ip: Option<String>,
    #[serde(rename = "startTime")]
    pub start_time: Option<String>,
    #[serde(rename = "containerStatuses")]
    pub container_statuses: Option<Vec<ContainerStatus>>,
    pub conditions: Option<Vec<Value>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ContainerStatus {
    pub name: Option<String>,
    pub ready: Option<bool>,
    #[serde(rename = "restartCount")]
    pub restart_count: Option<i64>,
    pub state: Option<Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Deployment {
    pub metadata: Option<Metadata>,
    pub spec: Option<Value>,
    pub status: Option<Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Network {
    pub metadata: Option<Metadata>,
    pub spec: Option<Value>,
    pub status: Option<Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BareMetalHost {
    pub metadata: Option<Metadata>,
    pub spec: Option<Value>,
    pub status: Option<Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Event {
    pub metadata: Option<Metadata>,
    #[serde(rename = "involvedObject")]
    pub involved_object: Option<Value>,
    pub reason: Option<String>,
    pub message: Option<String>,
    #[serde(rename = "type")]
    pub event_type: Option<String>,
    #[serde(rename = "lastTimestamp")]
    pub last_timestamp: Option<String>,
    pub count: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PVC {
    pub metadata: Option<Metadata>,
    pub spec: Option<Value>,
    pub status: Option<Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ISCSICdrom {
    pub metadata: Option<Metadata>,
    pub spec: Option<Value>,
    pub status: Option<Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ISCSIDisk {
    pub metadata: Option<Metadata>,
    pub spec: Option<Value>,
    pub status: Option<Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Job {
    pub metadata: Option<Metadata>,
    pub spec: Option<Value>,
    pub status: Option<Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct JobRunner {
    pub metadata: Option<Metadata>,
    pub spec: Option<Value>,
    pub status: Option<Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ItemList<T> {
    pub items: Option<Vec<T>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ConsistencyReport {
    pub summary: Option<Value>,
    pub checks: Option<Value>,
}

impl MkubeClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            client: Client::builder()
                .danger_accept_invalid_certs(true)
                .timeout(std::time::Duration::from_secs(10))
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
            .map_err(|e| format!("request failed: {}", e))?
            .json::<T>()
            .await
            .map_err(|e| format!("json parse failed: {}", e))
    }

    async fn get_text(&self, path: &str) -> Result<String, String> {
        let url = format!("{}{}", self.base_url, path);
        self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("request failed: {}", e))?
            .text()
            .await
            .map_err(|e| format!("text read failed: {}", e))
    }

    // Health
    pub async fn healthz(&self) -> Result<HealthResponse, String> {
        self.get_json("/healthz").await
    }

    // Nodes
    pub async fn list_nodes(&self) -> Result<ItemList<Node>, String> {
        self.get_json("/api/v1/nodes").await
    }

    // Pods
    pub async fn list_pods(&self) -> Result<ItemList<Pod>, String> {
        self.get_json("/api/v1/pods").await
    }

    pub async fn get_pod(&self, ns: &str, name: &str) -> Result<Pod, String> {
        self.get_json(&format!("/api/v1/namespaces/{}/pods/{}", ns, name)).await
    }

    pub async fn get_pod_logs(&self, ns: &str, name: &str) -> Result<String, String> {
        self.get_text(&format!("/api/v1/namespaces/{}/pods/{}/log", ns, name)).await
    }

    pub async fn delete_pod(&self, ns: &str, name: &str) -> Result<(), String> {
        let url = format!("{}/api/v1/namespaces/{}/pods/{}", self.base_url, ns, name);
        self.client.delete(&url).send().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    // Deployments
    pub async fn list_deployments(&self) -> Result<ItemList<Deployment>, String> {
        self.get_json("/api/v1/deployments").await
    }

    pub async fn get_deployment(&self, ns: &str, name: &str) -> Result<Deployment, String> {
        self.get_json(&format!("/api/v1/namespaces/{}/deployments/{}", ns, name)).await
    }

    pub async fn delete_deployment(&self, ns: &str, name: &str) -> Result<(), String> {
        let url = format!("{}/api/v1/namespaces/{}/deployments/{}", self.base_url, ns, name);
        self.client.delete(&url).send().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    // Networks
    pub async fn list_networks(&self) -> Result<ItemList<Network>, String> {
        self.get_json("/api/v1/networks").await
    }

    pub async fn get_network(&self, name: &str) -> Result<Network, String> {
        self.get_json(&format!("/api/v1/networks/{}", name)).await
    }

    pub async fn smoketest_network(&self, name: &str) -> Result<Value, String> {
        let url = format!("{}/api/v1/networks/{}/smoketest", self.base_url, name);
        self.client.post(&url).send().await
            .map_err(|e| e.to_string())?
            .json::<Value>().await
            .map_err(|e| e.to_string())
    }

    // DNS/DHCP proxy
    pub async fn list_dns_records(&self, net: &str) -> Result<ItemList<Value>, String> {
        self.get_json(&format!("/api/v1/namespaces/{}/dnsrecords", net)).await
    }

    pub async fn list_dhcp_pools(&self, net: &str) -> Result<ItemList<Value>, String> {
        self.get_json(&format!("/api/v1/namespaces/{}/dhcppools", net)).await
    }

    pub async fn list_dhcp_reservations(&self, net: &str) -> Result<ItemList<Value>, String> {
        self.get_json(&format!("/api/v1/namespaces/{}/dhcpreservations", net)).await
    }

    pub async fn list_dhcp_leases(&self, net: &str) -> Result<ItemList<Value>, String> {
        self.get_json(&format!("/api/v1/namespaces/{}/dhcpleases", net)).await
    }

    pub async fn list_dns_forwarders(&self, net: &str) -> Result<ItemList<Value>, String> {
        self.get_json(&format!("/api/v1/namespaces/{}/dnsforwarders", net)).await
    }

    // BMH
    pub async fn list_bmh(&self) -> Result<ItemList<BareMetalHost>, String> {
        self.get_json("/api/v1/baremetalhosts").await
    }

    pub async fn get_bmh(&self, ns: &str, name: &str) -> Result<BareMetalHost, String> {
        self.get_json(&format!("/api/v1/namespaces/{}/baremetalhosts/{}", ns, name)).await
    }

    pub async fn patch_bmh(&self, ns: &str, name: &str, patch: &Value) -> Result<(), String> {
        let url = format!("{}/api/v1/namespaces/{}/baremetalhosts/{}", self.base_url, ns, name);
        self.client.patch(&url)
            .header("Content-Type", "application/merge-patch+json")
            .json(patch)
            .send().await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    // PVCs
    pub async fn list_pvcs(&self) -> Result<ItemList<PVC>, String> {
        self.get_json("/api/v1/persistentvolumeclaims").await
    }

    // iSCSI CDROMs
    pub async fn list_iscsi_cdroms(&self) -> Result<ItemList<ISCSICdrom>, String> {
        self.get_json("/api/v1/iscsi-cdroms").await
    }

    // iSCSI Disks
    pub async fn list_iscsi_disks(&self) -> Result<ItemList<ISCSIDisk>, String> {
        self.get_json("/api/v1/iscsi-disks").await
    }

    pub async fn get_disk_capacity(&self) -> Result<Value, String> {
        self.get_json("/api/v1/iscsi-disks/capacity").await
    }

    // Jobs
    pub async fn list_jobs(&self) -> Result<ItemList<Job>, String> {
        self.get_json("/api/v1/namespaces/default/jobs").await
    }

    pub async fn list_job_runners(&self) -> Result<ItemList<JobRunner>, String> {
        self.get_json("/api/v1/jobrunners").await
    }

    pub async fn get_job_queue(&self) -> Result<Value, String> {
        self.get_json("/api/v1/jobqueue").await
    }

    pub async fn get_job_logs(&self, ns: &str, name: &str) -> Result<String, String> {
        self.get_text(&format!("/api/v1/namespaces/{}/jobs/{}/logs", ns, name)).await
    }

    pub async fn cancel_job(&self, ns: &str, name: &str) -> Result<(), String> {
        let url = format!("{}/api/v1/namespaces/{}/jobs/{}/cancel", self.base_url, ns, name);
        self.client.post(&url).send().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    // Events
    pub async fn list_events(&self) -> Result<ItemList<Event>, String> {
        self.get_json("/api/v1/events").await
    }

    // Consistency
    pub async fn consistency(&self) -> Result<Value, String> {
        self.get_json("/api/v1/consistency").await
    }

    // Lifecycle stats
    pub async fn lifecycle_stats(&self) -> Result<Value, String> {
        self.get_json("/api/v1/lifecycle/stats").await
    }
}
