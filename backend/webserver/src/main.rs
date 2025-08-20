mod api;
pub mod consts;
pub mod dto_game_renderer;
pub mod utils;
mod webserver_error;

#[cfg(not(feature = "publish"))]
use axum::http::HeaderValue;
use axum::{
    Router,
    extract::DefaultBodyLimit,
    routing::{
        any,
        get,
        post,
    },
};
use tower::ServiceBuilder;
#[cfg(not(feature = "publish"))]
use tower_http::cors::CorsLayer;
use tower_http::{
    compression::CompressionLayer,
    limit::RequestBodyLimitLayer,
    trace::TraceLayer,
};

use crate::api::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let layers = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new());

    #[cfg(not(feature = "publish"))]
    let layers = layers.layer(
        CorsLayer::new().allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap()),
    );

    let app = Router::new()
        .route("/api/compose-base-images", post(compose_base_images))
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(50 * 1024 * 1024))
        .route("/api/get-game-types", get(get_game_types))
        .route(
            "/api/get-showcase-attack-base-image",
            get(get_showcase_attack_base_image),
        )
        .route("/api/get-showcase-attack", get(get_showcase_attack))
        .route("/api/optimize-attack", any(optimize_attack))
        .route("/api/reverse-projection", post(reverse_projection))
        .layer(layers);

    if cfg!(feature = "publish") {
        let listener = tokio::net::UnixListener::bind("webserver.sock").unwrap();

        axum::serve(listener, app).await.unwrap();
    } else {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
            .await
            .unwrap();

        axum::serve(listener, app).await.unwrap();
    }
}
