use chrono::{DateTime, FixedOffset};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ConvertResult {
    pub amount: f64,
    pub from_currency: String,
    pub to_currency: String,
    pub result: f64,
    pub rate: f64,
    pub source: String,
    pub rate_updated_at: DateTime<FixedOffset>,
}
