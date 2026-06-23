use std::time::Duration;

use reqwest::{header, Client};

use crate::error::{Error, Result};
use crate::models::{BcvRate, ConvertResult, HistoryEntry, ParallelRate, Rate, Status};

const DEFAULT_BASE_URL: &str = "https://api.tasave.com";

pub struct TasaVE {
    http: Client,
    base_url: String,
}

impl TasaVE {
    pub fn new() -> Self {
        Self::build(None, DEFAULT_BASE_URL.to_owned())
    }

    pub fn with_key(api_key: impl Into<String>) -> Self {
        Self::build(Some(api_key.into()), DEFAULT_BASE_URL.to_owned())
    }

    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    fn build(api_key: Option<String>, base_url: String) -> Self {
        let mut headers = header::HeaderMap::new();
        if let Some(key) = api_key {
            let value = format!("Bearer {key}");
            headers.insert(
                header::AUTHORIZATION,
                value.parse().expect("API key must be ASCII"),
            );
        }
        let http = Client::builder()
            .timeout(Duration::from_secs(10))
            .default_headers(headers)
            .build()
            .expect("failed to build HTTP client");
        Self { http, base_url }
    }

    pub fn rates(&self) -> RatesEndpoint<'_> {
        RatesEndpoint(self)
    }

    pub fn convert(&self) -> ConvertBuilder<'_> {
        ConvertBuilder {
            inner: self,
            amount: None,
            from: None,
            to: None,
            source: "bcv".to_owned(),
        }
    }

    pub fn history(&self) -> HistoryEndpoint<'_> {
        HistoryEndpoint(self)
    }

    pub async fn status(&self) -> Result<Status> {
        self.get("/v1/status").await
    }

    async fn get<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T> {
        self.get_query(path, &[]).await
    }

    async fn get_query<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        query: &[(&str, &str)],
    ) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self.http.get(&url).query(query).send().await?;
        if !resp.status().is_success() {
            return Err(Error::Api {
                status: resp.status().as_u16(),
                message: resp.text().await.unwrap_or_default(),
            });
        }
        Ok(resp.json().await?)
    }
}

impl Default for TasaVE {
    fn default() -> Self {
        Self::new()
    }
}

pub struct RatesEndpoint<'a>(&'a TasaVE);

impl<'a> RatesEndpoint<'a> {
    pub async fn current(&self) -> Result<Rate> {
        self.0.get("/v1/rates").await
    }

    pub async fn bcv(&self) -> Result<BcvRate> {
        self.0.get("/v1/rates/bcv").await
    }

    pub async fn parallel(&self) -> Result<ParallelRate> {
        self.0.get("/v1/rates/parallel").await
    }
}

pub struct ConvertBuilder<'a> {
    inner: &'a TasaVE,
    amount: Option<f64>,
    from: Option<String>,
    to: Option<String>,
    source: String,
}

impl<'a> ConvertBuilder<'a> {
    pub fn amount(mut self, amount: f64) -> Self {
        self.amount = Some(amount);
        self
    }

    pub fn from(mut self, currency: impl Into<String>) -> Self {
        self.from = Some(currency.into());
        self
    }

    pub fn to(mut self, currency: impl Into<String>) -> Self {
        self.to = Some(currency.into());
        self
    }

    pub fn source(mut self, source: impl Into<String>) -> Self {
        self.source = source.into();
        self
    }

    pub async fn send(self) -> Result<ConvertResult> {
        let amount = self.amount.ok_or(Error::MissingParam("amount"))?;
        let from = self.from.ok_or(Error::MissingParam("from"))?;
        let to = self.to.ok_or(Error::MissingParam("to"))?;
        let amount_str = amount.to_string();
        self.inner
            .get_query(
                "/v1/convert",
                &[
                    ("amount", amount_str.as_str()),
                    ("from", from.as_str()),
                    ("to", to.as_str()),
                    ("source", self.source.as_str()),
                ],
            )
            .await
    }
}

pub struct HistoryEndpoint<'a>(&'a TasaVE);

impl<'a> HistoryEndpoint<'a> {
    pub async fn range(&self, from: &str, to: &str) -> Result<Vec<HistoryEntry>> {
        self.0
            .get_query("/v1/history", &[("from", from), ("to", to)])
            .await
    }

    pub async fn date(&self, date: &str) -> Result<HistoryEntry> {
        self.0.get(&format!("/v1/history/{date}")).await
    }
}
