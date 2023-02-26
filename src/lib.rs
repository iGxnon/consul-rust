#![allow(non_snake_case)]
#![allow(unused_doc_comments)]

pub mod agent;
pub mod catalog;
pub mod connect_ca;
pub mod errors;
pub mod health;
pub mod kv;
pub mod session;

mod request;

use std::env;

use std::time::Duration;

use reqwest::Client as HttpClient;
use reqwest::ClientBuilder;

use errors::Result;

#[derive(Clone, Debug)]
pub struct Client {
    config: Config,
}

impl Client {
    pub fn new(config: Config) -> Self {
        Client { config }
    }
}

#[derive(Clone, Debug)]
pub struct Config {
    pub address: String,
    pub datacenter: Option<String>,
    pub http_client: HttpClient,
    pub token: Option<String>,
    pub wait_time: Option<Duration>,
}

impl Config {
    pub fn new() -> Result<Config> {
        let conf = ClientBuilder::new().build().map(|client| Config {
            address: String::from("http://localhost:8500"),
            datacenter: None,
            http_client: client,
            token: None,
            wait_time: None,
        })?;
        Ok(conf)
    }

    pub fn new_from_env() -> Result<Config> {
        let consul_addr = match env::var("CONSUL_HTTP_ADDR") {
            Ok(val) => {
                if val.starts_with("http") {
                    val
                } else {
                    format!("http://{}", val)
                }
            }
            Err(_e) => String::from("http://127.0.0.1:8500"),
        };
        let consul_token = env::var("CONSUL_HTTP_TOKEN").ok();
        let conf = ClientBuilder::new().build().map(|client| Config {
            address: consul_addr,
            datacenter: None,
            http_client: client,
            token: consul_token,
            wait_time: None,
        })?;
        Ok(conf)
    }

    pub fn new_from_consul_host(
        host: &str,
        port: Option<u16>,
        token: Option<String>,
    ) -> Result<Config> {
        let conf = ClientBuilder::new().build().map(|client| Config {
            address: format!("{}:{}", host, port.unwrap_or(8500)),
            datacenter: None,
            http_client: client,
            token,
            wait_time: None,
        })?;
        Ok(conf)
    }

    pub fn new_from_addr(addr: &str, token: Option<String>) -> Result<Config> {
        let conf = ClientBuilder::new().build().map(|client| Config {
            address: addr.to_string(),
            datacenter: None,
            http_client: client,
            token,
            wait_time: None,
        })?;
        Ok(conf)
    }
}

#[derive(Clone, Debug, Default)]
pub struct QueryOptions {
    pub datacenter: Option<String>,
    pub wait_index: Option<u64>,
    pub wait_time: Option<Duration>,
}

#[derive(Clone, Debug)]
pub struct QueryMeta {
    pub last_index: Option<u64>,
    pub request_time: Duration,
}

#[derive(Clone, Debug, Default)]
pub struct WriteOptions {
    pub datacenter: Option<String>,
}

#[derive(Clone, Debug)]
pub struct WriteMeta {
    pub request_time: Duration,
}
