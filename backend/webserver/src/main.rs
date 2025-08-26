mod api;
mod webserver_error;

#[cfg(not(feature = "publish"))]
use axum::http::HeaderValue;
#[cfg(feature = "publish")]
use axum::response::Html;
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
use tower_http::compression::CompressionLayer;
#[cfg(not(feature = "publish"))]
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
};

use crate::api::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let layers = ServiceBuilder::new().layer(CompressionLayer::new());

    #[cfg(not(feature = "publish"))]
    let layers = layers.layer(TraceLayer::new_for_http()).layer(
        CorsLayer::new().allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap()),
    );

    let app = Router::new()
        .route("/api/compose-base-images", post(compose_base_images))
        .layer(DefaultBodyLimit::disable())
        .route("/api/get-game-types", get(get_game_types))
        .route(
            "/api/get-showcase-attack-base-image",
            get(get_showcase_attack_base_image),
        )
        .route("/api/get-showcase-attack", get(get_showcase_attack))
        .route("/api/optimize-attack", any(optimize_attack))
        .route("/api/reverse-projection", post(reverse_projection))
        .layer(layers);

    #[cfg(feature = "publish")]
    let app = app.route(
        "/",
        get(async || Html::<&str>::from(include_str!("../index.html"))),
    );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    println!("Listening at http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
