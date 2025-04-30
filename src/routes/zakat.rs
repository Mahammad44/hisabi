use axum::Json;
use crate::models::{ZakatRequest, ZakatResponse};

const GOLD_NISAB_GRAMS: f64 = 85.0;
const SILVER_NISAB_GRAMS: f64 = 595.0;
const GOLD_PRICE_PER_GRAM: f64 = 60.0;
const SILVER_PRICE_PER_GRAM: f64 = 0.7;

pub async fn calculate_zakat(Json(payload): Json<ZakatRequest>) -> Json<ZakatResponse> {
    let gold_value = payload.gold_grams * GOLD_PRICE_PER_GRAM;
    let silver_value = payload.silver_grams * SILVER_PRICE_PER_GRAM;
    let net_assets = payload.cash + gold_value + silver_value - payload.debts;

    let gold_threshold = GOLD_NISAB_GRAMS * GOLD_PRICE_PER_GRAM;
    let silver_threshold = SILVER_NISAB_GRAMS * SILVER_PRICE_PER_GRAM;

    let zakat_due = if net_assets >= gold_threshold {
        net_assets * 0.025
    } else if net_assets >= silver_threshold {
        net_assets * 0.025
    } else {
        0.0
    };

    let message = if zakat_due > 0.0 {
        if net_assets >= gold_threshold {
            format!("You owe ${:.2} based on gold nisab (85 g).", zakat_due)
        } else {
            format!("You owe ${:.2} based on silver nisab (595 g).", zakat_due)
        }
    } else {
        "No zakat due: your net assets did not meet either nisab threshold.".into()
    };

    Json(ZakatResponse { zakat_due, message })
}
