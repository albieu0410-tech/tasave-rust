use chrono::{DateTime, FixedOffset};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Status {
    pub status: String,
    pub last_updated: DateTime<FixedOffset>,
    pub confidence: f64,
    pub verified: bool,
    pub sources: Vec<String>,
    pub is_preliminary: bool,
    pub stale: bool,
}
