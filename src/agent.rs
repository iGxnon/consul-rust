use async_trait::async_trait;
use std::collections::HashMap;

use crate::errors::Result;
use crate::request::{get, put};
use crate::Client;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct AgentCheck {
    pub Node: String,
    pub CheckID: String,
    pub Name: String,
    pub Status: String,
    pub Notes: String,
    pub Output: String,
    pub ServiceID: String,
    pub ServiceName: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct TaggedAddress {
    pub Address: String,
    pub Port: u16,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct TaggedAddresses {
    pub lan_ipv4: TaggedAddress,
    pub wan_ipv4: TaggedAddress,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct RegisterAgentCheck {
    pub Name: String,
    pub ID: String,
    pub Interval: String,
    pub Notes: String,
    pub DeregisterCriticalServiceAfter: String,
    pub Timeout: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub AliasNode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub AliasService: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub OutputMaxSize: Option<usize>,
    // Script check
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Args: Option<Vec<String>>,
    // Docker check
    #[serde(skip_serializing_if = "Option::is_none")]
    pub DockerContainerID: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Shell: Option<String>,
    // GRPC check
    #[serde(skip_serializing_if = "Option::is_none")]
    pub GRPC: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub GRPCUseTLS: Option<bool>,
    // Http2 ping check
    #[serde(skip_serializing_if = "Option::is_none")]
    pub H2PING: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub H2PingUseTLS: Option<bool>,
    // Http query check
    pub HTTP: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub DisableRedirects: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Header: Option<HashMap<String, Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub TLSServerName: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub TLSSkipVerify: Option<bool>,
    // TCP check
    #[serde(skip_serializing_if = "Option::is_none")]
    pub TCP: Option<String>,
    // UDP check
    #[serde(skip_serializing_if = "Option::is_none")]
    pub UDP: Option<String>,
    // OS service check
    #[serde(skip_serializing_if = "Option::is_none")]
    pub OSService: Option<String>,
    // TTL check
    #[serde(skip_serializing_if = "Option::is_none")]
    pub TTL: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ServiceID: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Status: Option<String>,
    // Since Consul 1.7.0/1.11.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub SuccessBeforePassing: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub FailuresBeforeWarning: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub FailuresBeforeCritical: Option<i32>,
}

pub enum TTLStatus {
    PASS,
    WARN,
    FAIL,
}

impl TTLStatus {
    fn path(&self) -> &str {
        match self {
            TTLStatus::PASS => "pass",
            TTLStatus::WARN => "warn",
            TTLStatus::FAIL => "fail",
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct AgentMember {
    pub Name: String,
    pub Addr: String,
    pub Port: u16,
    pub Tags: HashMap<String, String>,
    pub pubStatus: usize,
    pub ProtocolMin: u8,
    pub ProtocolMax: u8,
    pub ProtocolCur: u8,
    pub DelegateMin: u8,
    pub DelegateMax: u8,
    pub DelegateCur: u8,
}

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct AgentService {
    pub ID: String,
    pub Service: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Tags: Option<Vec<String>>,
    pub Port: u16,
    pub Address: String,
    pub EnableTagOverride: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub CreateIndex: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ModifyIndex: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Weights: Option<HashMap<String, i32>>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct RegisterAgentService {
    pub Name: String,
    pub ID: String,
    pub Address: String,
    pub Port: u16,
    pub Kind: String,
    pub EnableTagOverride: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub TaggedAddresses: Option<TaggedAddresses>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Meta: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Check: Option<AgentCheck>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Weights: Option<HashMap<String, i32>>,
}

#[async_trait]
pub trait Agent {
    async fn members(&self, wan: bool) -> Result<AgentMember>;
    async fn reload(&self) -> Result<()>;
    async fn maintenance_mode(&self, enable: bool, reason: Option<&str>) -> Result<()>;
    async fn join(&self, address: &str, wan: bool) -> Result<()>;
    async fn leave(&self) -> Result<()>;
    async fn force_leave(&self) -> Result<()>;
    async fn checks(&self) -> Result<HashMap<String, AgentCheck>>;
    async fn register_check(&self, reg: &RegisterAgentCheck) -> Result<()>;
    async fn deregister_check(&self, check_id: &str) -> Result<()>;
    async fn ttl_check_send(
        &self,
        status: TTLStatus,
        check_id: &str,
        note: Option<&str>,
    ) -> Result<()>;
    async fn services(&self, filter: Option<&str>) -> Result<HashMap<String, AgentService>>;
    async fn register_service(
        &self,
        reg: &RegisterAgentService,
        replace_existing_checks: bool,
    ) -> Result<()>;
    async fn deregister_service(&self, service_id: &str) -> Result<()>;
    async fn service_maintenance_mode(
        &self,
        service_id: &str,
        enable: bool,
        reason: Option<&str>,
    ) -> Result<()>;
}

#[async_trait]
impl Agent for Client {
    /// https://www.consul.io/api/agent.html#list-members
    async fn members(&self, wan: bool) -> Result<AgentMember> {
        let mut params = HashMap::new();
        if wan {
            params.insert(String::from("wan"), String::from("1"));
        }
        get("/v1/agent/members", &self.config, params, None)
            .await
            .map(|x| x.0)
    }

    /// https://www.consul.io/api/agent.html#reload-agent
    async fn reload(&self) -> Result<()> {
        put(
            "/v1/agent/reload",
            None as Option<&()>,
            &self.config,
            HashMap::new(),
            None,
        )
        .await
        .map(|x| x.0)
    }

    /// https://www.consul.io/api/agent.html#reload-agent
    async fn maintenance_mode(&self, enable: bool, reason: Option<&str>) -> Result<()> {
        let mut params = HashMap::new();
        let enable_str = if enable {
            String::from("true")
        } else {
            String::from("false")
        };
        params.insert(String::from("enabled"), enable_str);
        if let Some(r) = reason {
            params.insert(String::from("reason"), r.to_owned());
        }
        put(
            "/v1/agent/maintenance",
            None as Option<&()>,
            &self.config,
            params,
            None,
        )
        .await
        .map(|x| x.0)
    }

    ///https://www.consul.io/api/agent.html#join-agent
    async fn join(&self, address: &str, wan: bool) -> Result<()> {
        let mut params = HashMap::new();

        if wan {
            params.insert(String::from("wan"), String::from("true"));
        }
        let path = format!("/v1/agent/join/{}", address);
        put(&path, None as Option<&()>, &self.config, params, None)
            .await
            .map(|x| x.0)
    }

    /// https://www.consul.io/api/agent.html#graceful-leave-and-shutdown
    async fn leave(&self) -> Result<()> {
        put(
            "/v1/agent/leave",
            None as Option<&()>,
            &self.config,
            HashMap::new(),
            None,
        )
        .await
        .map(|x| x.0)
    }

    ///https://www.consul.io/api/agent.html#force-leave-and-shutdown
    async fn force_leave(&self) -> Result<()> {
        put(
            "/v1/agent/force-leave",
            None as Option<&()>,
            &self.config,
            HashMap::new(),
            None,
        )
        .await
        .map(|x| x.0)
    }

    /// https://www.consul.io/api/agent/check.html#list-checks
    async fn checks(&self) -> Result<HashMap<String, AgentCheck>> {
        get("/v1/agent/checks", &self.config, HashMap::new(), None)
            .await
            .map(|x| x.0)
    }

    /// https://developer.hashicorp.com/consul/api-docs/agent/check#register-check
    async fn register_check(&self, reg: &RegisterAgentCheck) -> Result<()> {
        put(
            "/v1/agent/check/register",
            Some(reg),
            &self.config,
            HashMap::new(),
            None,
        )
        .await
        .map(|x| x.0)
    }

    /// https://developer.hashicorp.com/consul/api-docs/agent/check#deregister-check
    async fn deregister_check(&self, check_id: &str) -> Result<()> {
        put(
            &format!("/v1/agent/check/deregister/{}", check_id),
            None as Option<&()>,
            &self.config,
            HashMap::new(),
            None,
        )
        .await
        .map(|x| x.0)
    }

    /// https://developer.hashicorp.com/consul/api-docs/agent/check#ttl-check-xxx
    async fn ttl_check_send(
        &self,
        status: TTLStatus,
        check_id: &str,
        note: Option<&str>,
    ) -> Result<()> {
        let mut params = HashMap::new();
        if let Some(note) = note {
            params.insert("note".to_string(), note.to_string());
        }
        put(
            &format!("/v1/agent/check/{}/{}", status.path(), check_id),
            None as Option<&()>,
            &self.config,
            params,
            None,
        )
        .await
        .map(|x| x.0)
    }

    /// https://developer.hashicorp.com/consul/api-docs/agent/service#list-services
    async fn services(&self, filter: Option<&str>) -> Result<HashMap<String, AgentService>> {
        let mut params = HashMap::new();
        if let Some(filter) = filter {
            params.insert("filter".to_string(), filter.to_string());
        }
        get("/v1/agent/services", &self.config, params, None)
            .await
            .map(|x| x.0)
    }

    /// https://developer.hashicorp.com/consul/api-docs/agent/service#register-service
    async fn register_service(
        &self,
        reg: &RegisterAgentService,
        replace_existing_checks: bool,
    ) -> Result<()> {
        put(
            &format!(
                "/v1/agent/service/register?replace-existing-checks={}",
                replace_existing_checks
            ),
            Some(reg),
            &self.config,
            HashMap::new(),
            None,
        )
        .await
        .map(|x| x.0)
    }

    /// https://developer.hashicorp.com/consul/api-docs/agent/service#deregister-service
    async fn deregister_service(&self, service_id: &str) -> Result<()> {
        put(
            &format!("/v1/agent/service/deregister/{}", service_id),
            None as Option<&()>,
            &self.config,
            HashMap::new(),
            None,
        )
        .await
        .map(|x| x.0)
    }

    /// https://developer.hashicorp.com/consul/api-docs/agent/service#enable-maintenance-mode
    async fn service_maintenance_mode(
        &self,
        service_id: &str,
        enable: bool,
        reason: Option<&str>,
    ) -> Result<()> {
        let mut params = HashMap::new();
        let enable_str = if enable {
            String::from("true")
        } else {
            String::from("false")
        };
        params.insert(String::from("enabled"), enable_str);
        if let Some(r) = reason {
            params.insert(String::from("reason"), r.to_owned());
        }
        put(
            &format!("/v1/agent/service/maintenance/{}", service_id),
            None as Option<&()>,
            &self.config,
            params,
            None,
        )
        .await
        .map(|x| x.0)
    }
}
