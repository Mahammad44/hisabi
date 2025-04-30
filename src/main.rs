use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::net::TcpListener;                   // ← import the Tokio TCP listener

// ─── Your constants ────────────────────────────────────────────────────────────
const GOLD_NISAB_GRAMS: f64   = 85.0;
const SILVER_NISAB_GRAMS: f64 = 595.0;
const GOLD_PRICE_PER_GRAM: f64   = 60.0;
const SILVER_PRICE_PER_GRAM: f64 = 0.7;

// ─── Your request/response types ───────────────────────────────────────────────
#[derive(Deserialize)]
struct ZakatRequest {
    cash:          f64,
    gold_grams:    f64,
    silver_grams:  f64,
    debts:         f64,
}

#[derive(Serialize)]
struct ZakatResponse {
    zakat_due: f64,
    message:   String,
}

// ─── Your handler ───────────────────────────────────────────────────────────────
async fn calculate_zakat(Json(payload): Json<ZakatRequest>) -> Json<ZakatResponse> {
    let ZakatRequest { cash, gold_grams, silver_grams, debts } = payload;

    let gold_value   = gold_grams   * GOLD_PRICE_PER_GRAM;
    let silver_value = silver_grams * SILVER_PRICE_PER_GRAM;

    // check gold nisab first
    let zakat_on_gold   = if gold_value >= GOLD_NISAB_GRAMS   * GOLD_PRICE_PER_GRAM {
        gold_value * 0.025
    } else { 0.0 };

    // if gold nisab not met, check silver
    let zakat_on_silver = if gold_value < GOLD_NISAB_GRAMS * GOLD_PRICE_PER_GRAM
                         && silver_value >= SILVER_NISAB_GRAMS * SILVER_PRICE_PER_GRAM
    {
        silver_value * 0.025
    } else { 0.0 };

    let zakat_due = if zakat_on_gold > 0.0 {
        zakat_on_gold
    } else {
        zakat_on_silver
    };

    let message = if zakat_due > 0.0 {
        if zakat_on_gold > 0.0 {
            format!("You owe ${:.2} based on the gold nisab.", zakat_due)
        } else {
            format!(
                "You owe ${:.2} based on the silver nisab. You didn’t meet the gold threshold but did meet the silver one.",
                zakat_due
            )
        }
    } else {
        "No zakat due: your wealth did not meet either nisab threshold.".into()
    };

    Json(ZakatResponse { zakat_due, message })
}

// ─── Main & server setup ───────────────────────────────────────────────────────
#[tokio::main]
async fn main() {
    // build our application with a single POST /calculate route
    let app = Router::new().route("/calculate", post(calculate_zakat));

    // bind to 127.0.0.1:3000
    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("failed to bind");

    println!("Server running at http://127.0.0.1:3000");

    // start serving — use the free `axum::serve` function
    axum::serve(listener, app).await.expect("server error");
}

