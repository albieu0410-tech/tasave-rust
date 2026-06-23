use chrono::{DateTime, FixedOffset, NaiveDate};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Rate {
    pub bcv_usd: f64,
    pub bcv_eur: Option<f64>,
    pub parallel_usdt: Option<f64>,
    pub parallel_buy: Option<f64>,
    pub parallel_sell: Option<f64>,
    pub confidence: f64,
    pub verified: bool,
    pub checked_against: Vec<String>,
    pub valid_from: DateTime<FixedOffset>,
    pub valid_until: DateTime<FixedOffset>,
    pub next_expected_update: DateTime<FixedOffset>,
    pub next_business_day: NaiveDate,
    pub is_preliminary: bool,
    pub official_since: Option<DateTime<FixedOffset>>,
    pub published_at: Option<DateTime<FixedOffset>>,
    pub sources: Vec<String>,
    pub consensus: bool,
    pub updated_at: DateTime<FixedOffset>,
    pub stale: Option<bool>,
    pub stale_since: Option<DateTime<FixedOffset>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BcvRate {
    pub bcv_usd: f64,
    pub bcv_eur: Option<f64>,
    pub confidence: f64,
    pub verified: bool,
    pub valid_from: DateTime<FixedOffset>,
    pub valid_until: DateTime<FixedOffset>,
    pub next_expected_update: DateTime<FixedOffset>,
    pub next_business_day: NaiveDate,
    pub is_preliminary: bool,
    pub official_since: Option<DateTime<FixedOffset>>,
    pub published_at: Option<DateTime<FixedOffset>>,
    pub sources: Vec<String>,
    pub updated_at: DateTime<FixedOffset>,
    pub stale: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ParallelRate {
    pub parallel_usdt: Option<f64>,
    pub parallel_buy: Option<f64>,
    pub parallel_sell: Option<f64>,
    pub sources: Vec<String>,
    pub updated_at: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HistoryEntry {
    pub date: NaiveDate,
    pub bcv_usd: f64,
    pub bcv_eur: Option<f64>,
    pub parallel_usdt: Option<f64>,
    pub confidence: f64,
    pub sources: Vec<String>,
}
