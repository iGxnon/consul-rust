use std::collections::HashMap;
use url::Url;

use std::str;
use std::str::FromStr;
use std::time::Instant;

use reqwest::header::HeaderValue;
use reqwest::RequestBuilder;
use reqwest::{Client as HttpClient, StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::errors::Result;
use crate::{Config, QueryMeta, QueryOptions, WriteMeta, WriteOptions};

fn add_config_options(builder: RequestBuilder, config: &Config) -> RequestBuilder {
    match &config.token {
        Some(val) => builder.header("X-Consul-Token", val),
        None => builder,
    }
}

pub async fn get_vec<R: DeserializeOwned>(
    path: &str,
    config: &Config,
    mut params: HashMap<String, String>,
    options: Option<&QueryOptions>,
) -> Result<(Vec<R>, QueryMeta)> {
    let datacenter: Option<&String> = options
        .and_then(|o| o.datacenter.as_ref())
        .or(config.datacenter.as_ref());
    if let Some(dc) = datacenter {
        params.insert(String::from("dc"), dc.to_owned());
    }
    if let Some(options) = options {
        if let Some(index) = options.wait_index {
            params.insert(String::from("index"), index.to_string());
        }
        if let Some(wait_time) = options.wait_time {
            params.insert(String::from("wait"), format!("{}s", wait_time.as_secs()));
        }
    }
    let url_str = format!("{}{}", config.address, path);
    let url = Url::parse_with_params(&url_str, params.iter())?;
    let start = Instant::now();
    let request_builder = add_config_options(config.http_client.get(url), config);
    let response = request_builder.send().await?;
    let code = response.status();
    let last_index = response
        .headers()
        .get("X-Consul-Index")
        .and_then(|value: &HeaderValue| value.to_str().ok())
        .map(u64::from_str)
        .transpose()?;
    if code == StatusCode::NOT_FOUND {
        return Ok((
            Vec::new(),
            QueryMeta {
                last_index,
                request_time: Instant::now() - start,
            },
        ));
    }
    response.error_for_status_ref()?;
    let payload: Vec<_> = response.json().await?;
    Ok((
        payload,
        QueryMeta {
            last_index,
            request_time: Instant::now() - start,
        },
    ))
}

pub async fn get<R: DeserializeOwned>(
    path: &str,
    config: &Config,
    mut params: HashMap<String, String>,
    options: Option<&QueryOptions>,
) -> Result<(R, QueryMeta)> {
    let datacenter: Option<&String> = options
        .and_then(|o| o.datacenter.as_ref())
        .or(config.datacenter.as_ref());
    if let Some(dc) = datacenter {
        params.insert(String::from("dc"), dc.to_owned());
    }
    if let Some(options) = options {
        if let Some(index) = options.wait_index {
            params.insert(String::from("index"), index.to_string());
        }
        if let Some(wait_time) = options.wait_time {
            params.insert(String::from("wait"), format!("{}s", wait_time.as_secs()));
        }
    }
    let url_str = format!("{}{}", config.address, path);
    let url = Url::parse_with_params(&url_str, params.iter())?;
    let start = Instant::now();
    let request_builder = add_config_options(config.http_client.get(url), config);
    let response = request_builder.send().await?;
    response.error_for_status_ref()?;
    let last_index = response
        .headers()
        .get("X-Consul-Index")
        .and_then(|value: &HeaderValue| value.to_str().ok())
        .map(u64::from_str)
        .transpose()?;
    let payload = response.json().await?;
    Ok((
        payload,
        QueryMeta {
            last_index,
            request_time: Instant::now() - start,
        },
    ))
}

pub async fn delete<R: DeserializeOwned>(
    path: &str,
    config: &Config,
    params: HashMap<String, String>,
    options: Option<&WriteOptions>,
) -> Result<(R, WriteMeta)> {
    let req = |http_client: &HttpClient, url: Url| -> RequestBuilder { http_client.delete(url) };
    write_with_body(path, None as Option<&()>, config, params, options, req).await
}

pub async fn put<T: Serialize, R: DeserializeOwned>(
    path: &str,
    body: Option<&T>,
    config: &Config,
    params: HashMap<String, String>,
    options: Option<&WriteOptions>,
) -> Result<(R, WriteMeta)> {
    let req = |http_client: &HttpClient, url: Url| -> RequestBuilder { http_client.put(url) };
    write_with_body(path, body, config, params, options, req).await
}

async fn write_with_body<T: Serialize, R: DeserializeOwned, F>(
    path: &str,
    body: Option<&T>,
    config: &Config,
    mut params: HashMap<String, String>,
    options: Option<&WriteOptions>,
    req: F,
) -> Result<(R, WriteMeta)>
where
    F: Fn(&HttpClient, Url) -> RequestBuilder,
{
    let start = Instant::now();
    let datacenter: Option<&String> = options
        .and_then(|o| o.datacenter.as_ref())
        .or(config.datacenter.as_ref());
    if let Some(dc) = datacenter {
        params.insert(String::from("dc"), dc.to_owned());
    }
    let url_str = format!("{}{}", config.address, path);
    let url = Url::parse_with_params(&url_str, params.iter())?;
    let builder = req(&config.http_client, url);
    let builder = if let Some(b) = body {
        builder.json(b)
    } else {
        builder
    };
    let builder = add_config_options(builder, config);
    let x = builder.send().await?.error_for_status()?.json().await?;
    Ok((
        x,
        WriteMeta {
            request_time: Instant::now() - start,
        },
    ))
}
