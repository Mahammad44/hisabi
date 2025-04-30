use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::net::TcpListener;

// ─── Global Constants ─────────────────────────────────────────────────────────
const GOLD_NISAB_GRAMS: f64   = 85.0;   // 85 grams of gold
const SILVER_NISAB_GRAMS: f64 = 595.0;  // 595 grams of silver
const GOLD_PRICE_PER_GRAM: f64   = 60.0; // e.g. $60 per gram
const SILVER_PRICE_PER_GRAM: f64 = 0.7;  // e.g. $0.70 per gram

// ─── Request / Response Structs ───────────────────────────────────────────────
#[derive(Deserialize)]
struct ZakatRequest {
    cash:         f64,
    gold_grams:   f64,
    silver_grams: f64,
    debts:        f64,
}

#[derive(Serialize)]
struct ZakatResponse {
    zakat_due: f64,
    message:   String,
}

// ─── Handler ───────────────────────────────────────────────────────────────────
async fn calculate_zakat(Json(payload): Json<ZakatRequest>) -> Json<ZakatResponse> {
    // Destructure incoming data
    let ZakatRequest { cash, gold_grams, silver_grams, debts } = payload;

    // Compute values
    let gold_value   = gold_grams   * GOLD_PRICE_PER_GRAM;
    let silver_value = silver_grams * SILVER_PRICE_PER_GRAM;
    let net_assets   = cash + gold_value + silver_value - debts;

    // Compute thresholds in currency
    let gold_threshold   = GOLD_NISAB_GRAMS   * GOLD_PRICE_PER_GRAM;
    let silver_threshold = SILVER_NISAB_GRAMS * SILVER_PRICE_PER_GRAM;

    // Determine zakat based on net assets & thresholds
    let zakat_due = if net_assets >= gold_threshold {
        net_assets * 0.025
    } else if net_assets >= silver_threshold {
        net_assets * 0.025
    } else {
        0.0
    };

    // Craft response message
    let message = if zakat_due > 0.0 {
        if net_assets >= gold_threshold {
            format!("You owe ${:.2} based on gold nisab (85 g).", zakat_due)
        } else {
            format!(
                "You owe ${:.2} based on silver nisab (595 g). You did not meet gold threshold but did meet silver.",
                zakat_due
            )
        }
    } else {
        "No zakat due: your net assets did not meet either nisab threshold.".into()
    };

    Json(ZakatResponse { zakat_due, message })
}

// ─── Main & Server Setup ───────────────────────────────────────────────────────
#[tokio::main]
async fn main() {
    // Build our router with a single POST /calculate route
    let app = Router::new().route("/calculate", post(calculate_zakat));

    // Bind to localhost:3000
    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("failed to bind");

    println!("Server running at http://127.0.0.1:3000");

    // Serve the app
    axum::serve(listener, app)
        .await
        .expect("server error");
}

