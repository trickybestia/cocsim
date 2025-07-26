use axum::{
    http::{
        HeaderMap,
        header,
    },
    response::IntoResponse,
};
use cocsim::utils::load_test_map_raw;

use crate::consts::SHOWCASE_MAP;

pub async fn get_showcase_attack_base_image() -> impl IntoResponse {
    let (_, base_image) =
        load_test_map_raw(SHOWCASE_MAP).expect("Map should be loaded successfully");

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        "image/jpeg".parse().expect("Should be valid content type"),
    );

    (headers, base_image)
}
