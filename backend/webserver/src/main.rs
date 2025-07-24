mod api;
pub mod consts;
pub mod dto_game_renderer;
pub mod utils;

use axum::{
    Router,
    extract::DefaultBodyLimit,
    http::HeaderValue,
    routing::{
        get,
        post,
    },
};
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    cors::CorsLayer,
    limit::RequestBodyLimitLayer,
    trace::TraceLayer,
};

use crate::api::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/api/compose-base-images", post(compose_base_images))
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(50 * 1024 * 1024))
        .route("/api/get-building-types", get(get_building_types))
        .route(
            "/api/get-showcase-attack-base-image",
            get(get_showcase_attack_base_image),
        )
        .route("/api/get-showcase-attack", get(get_showcase_attack))
        .route("/api/get-unit-types", get(get_unit_types))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(
                    CorsLayer::new()
                        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap()),
                )
                .layer(CompressionLayer::new()),
        );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
