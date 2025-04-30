use axum::{routing::post, Router};
use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use tower_http::cors::{AllowOrigin, CorsLayer};

mod app;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(app::App);

    // Configure CORS properly
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::any())
        .allow_methods([axum::http::Method::GET, axum::http::Method::POST])
        .allow_headers(vec![
            axum::http::header::CONTENT_TYPE,
            axum::http::header::AUTHORIZATION,
            axum::http::header::ACCEPT,
        ]);

    let app = Router::new()
        .route("/api/calculate", post(routes::zakat::calculate_zakat))
        .leptos_routes(&leptos_options, routes, app::App)
        .layer(cors)
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("🚀 Server running on http://{}", addr);
    
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
