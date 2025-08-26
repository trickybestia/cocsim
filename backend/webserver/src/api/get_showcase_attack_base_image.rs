use axum::{
    http::{
        HeaderMap,
        header,
    },
    response::IntoResponse,
};

pub async fn get_showcase_attack_base_image() -> impl IntoResponse {
    let base_image = api_base::get_showcase_attack_base_image();

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        "image/jpeg".parse().expect("Should be valid content type"),
    );

    (headers, base_image)
}
