use axum::{
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Deserialize)]
struct ZakatInput {
    cash: f64,
    gold_grams: f64,
    debts: f64,
}

#[derive(Serialize)]
struct ZakatResult {
    zakat_due: f64,
    nisab_threshold: f64,
    is_obligatory: bool,
}

// Handler for POST /calculate
async fn calculate_zakat(Json(input): Json<ZakatInput>) -> Json<ZakatResult> {
    // Let's say the Nisab is the value of 85 grams of gold
    let gold_price_per_gram = 70.0; // USD
    let nisab = 85.0 * gold_price_per_gram;

    let total_assets = input.cash + (input.gold_grams * gold_price_per_gram);
    let net_assets = total_assets - input.debts;

    let zakat_due = if net_assets >= nisab {
        net_assets * 0.025 // 2.5%
    } else {
        0.0
    };

    Json(ZakatResult {
        zakat_due,
        nisab_threshold: nisab,
        is_obligatory: net_assets >= nisab,
    })
}

#[tokio::main]
async fn main() {
    // Build the app with a single route
    let app = Router::new().route("/calculate", post(calculate_zakat));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

