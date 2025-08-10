use reqwest::{
    header::{HeaderName, HeaderValue},
    Client, ClientBuilder, Response,
};
use std::{collections::HashMap, str::FromStr, time::Duration};
use tracing::instrument;

use super::error::HttpClientError;

#[derive(Clone, Debug)]
pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    #[instrument]
    pub async fn get(&self, url: String) -> Result<Response, HttpClientError> {
        Ok(self.client.get(url).send().await?)
    }

    #[instrument]
    pub async fn post(&self, url: String) -> Result<Response, HttpClientError> {
        Ok(self.client.post(url).send().await?)
    }

    #[instrument]
    pub async fn put(&self, url: String) -> Result<Response, HttpClientError> {
        Ok(self.client.put(url).send().await?)
    }

    #[instrument]
    pub async fn delete(&self, url: String) -> Result<Response, HttpClientError> {
        Ok(self.client.delete(url).send().await?)
    }
}

#[derive(Default)]
pub struct HttpClientBuilder {
    builder: ClientBuilder,
}

impl HttpClientBuilder {
    pub fn new() -> Self {
        Self {
            builder: Client::builder(),
        }
    }

    pub fn timeout(mut self, ttl: Duration) -> Self {
        self.builder = self.builder.timeout(ttl);
        self
    }

    pub fn default_headers(
        mut self,
        headers: HashMap<String, String>,
    ) -> Result<Self, HttpClientError> {
        let mut default_headers = reqwest::header::HeaderMap::new();
        for (key, value) in headers {
            let header_name = HeaderName::from_str(key.as_str())
                .map_err(|_| HttpClientError::InvalidUrl(format!("Invalid header key: {}", key)))?;
            let header_value = HeaderValue::from_str(value.as_str()).map_err(|_| {
                HttpClientError::InvalidUrl(format!("Invalid header value: {}", value))
            })?;

            default_headers.insert(header_name, header_value);
        }
        self.builder = self.builder.default_headers(default_headers);
        Ok(self)
    }

    pub fn build(self) -> Result<HttpClient, HttpClientError> {
        let client = self
            .builder
            .build()
            .map_err(|_| HttpClientError::InitilizingError)?;
        Ok(HttpClient { client })
    }
}
