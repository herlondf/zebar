use std::collections::HashMap;

use async_trait::async_trait;
use reqwest::{Client, Method};
use serde::{Deserialize, Serialize};

use crate::{
  common::AsyncInterval,
  providers::{
    CommonProviderState, Provider, ProviderInputMsg, RuntimeType,
  },
};

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpProviderConfig {
  /// URL to request.
  pub url: String,

  /// HTTP method to use. Defaults to `GET`.
  pub method: Option<String>,

  /// Headers to send with the request.
  #[serde(default)]
  pub headers: HashMap<String, String>,

  /// How often the request is made, in milliseconds.
  pub refresh_interval: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpOutput {
  /// HTTP status code.
  pub status: u16,

  /// Whether the status code is in the 200-299 range.
  pub success: bool,

  /// Response body as text.
  pub body: String,

  /// Response body parsed as JSON, or `None` if it is not JSON.
  ///
  /// Saves the widget a `JSON.parse` on every emission, and means a
  /// malformed response shows up as `null` rather than as an exception in
  /// the widget.
  pub json: Option<serde_json::Value>,
}

pub struct HttpProvider {
  config: HttpProviderConfig,
  common: CommonProviderState,
  client: Client,
}

impl HttpProvider {
  pub fn new(
    config: HttpProviderConfig,
    common: CommonProviderState,
  ) -> HttpProvider {
    HttpProvider {
      config,
      common,
      client: Client::new(),
    }
  }

  async fn run_interval(&self) -> anyhow::Result<HttpOutput> {
    let method = match &self.config.method {
      Some(method) => {
        Method::from_bytes(method.to_uppercase().as_bytes())?
      }
      None => Method::GET,
    };

    let mut request = self.client.request(method, &self.config.url);

    for (key, value) in &self.config.headers {
      request = request.header(key, value);
    }

    let response = request.send().await?;
    let status = response.status();
    let body = response.text().await?;

    Ok(HttpOutput {
      status: status.as_u16(),
      success: status.is_success(),
      json: serde_json::from_str(&body).ok(),
      body,
    })
  }
}

#[async_trait]
impl Provider for HttpProvider {
  fn runtime_type(&self) -> RuntimeType {
    RuntimeType::Async
  }

  async fn start_async(&mut self) {
    let mut interval = AsyncInterval::new(self.config.refresh_interval);

    loop {
      tokio::select! {
        _ = interval.tick() => {
          let output = self.run_interval().await;
          self.common.emitter.emit_output(output);
        }
        Some(input) = self.common.input.async_rx.recv() => {
          if let ProviderInputMsg::Stop = input {
            break;
          }
        }
      }
    }
  }
}
