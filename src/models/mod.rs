use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ZakatRequest {
    pub cash: f64,
    pub gold_grams: f64,
    pub silver_grams: f64,
    pub debts: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ZakatResponse {
    pub zakat_due: f64,
    pub message: String,
}
