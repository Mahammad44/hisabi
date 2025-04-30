use axum::{Router, routing::post, Json};
use serde::{Deserialize, Serialize};

pub fn zakat_routes() -> Router {
    Router::new().route("/calculate", post(calculate_zakat))
}

#[derive(Deserialize)]
pub struct ZakatRequest {
    pub cash: f64,
    pub gold_grams: f64,
    pub debts: f64,
}

#[derive(Serialize)]
pub struct ZakatResponse {
    pub zakat_due: f64,
    pub nisab_threshold: f64,
    pub is_obligatory: bool,
}

async fn calculate_zakat(Json(payload): Json<ZakatRequest>) -> Json<ZakatResponse> {
    let gold_price_per_gram = 59.5; // Static for now, could be dynamic later
    let nisab_threshold = gold_price_per_gram * 100.0;

    let net_assets = payload.cash + (payload.gold_grams * gold_price_per_gram) - payload.debts;
    let is_obligatory = net_assets >= nisab_threshold;
    let zakat_due = if is_obligatory { net_assets * 0.025 } else { 0.0 };

    Json(ZakatResponse {
        zakat_due,
        nisab_threshold,
        is_obligatory,
    })
}

