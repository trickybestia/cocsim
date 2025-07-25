use std::io::Cursor;

use axum::{
    extract::Multipart,
    http::{
        HeaderMap,
        StatusCode,
        header,
    },
    response::IntoResponse,
};
use image::codecs::jpeg::JpegEncoder;

fn reverse_projection_internal(image: &[u8]) -> anyhow::Result<Vec<u8>> {
    let result = compose_base_images::reverse_projection(image)?;
    let mut result_writer = Cursor::new(Vec::new());

    let mut encoder = JpegEncoder::new_with_quality(&mut result_writer, 70);

    encoder.encode_image(&result)?;

    Ok(result_writer.into_inner())
}

pub async fn reverse_projection(mut multipart: Multipart) -> impl IntoResponse {
    let file = multipart.next_field().await.unwrap().unwrap();
    let file_bytes = file.bytes().await.unwrap();

    if let Ok(result) = reverse_projection_internal(&file_bytes) {
        let mut headers = HeaderMap::new();
        headers.insert(header::CONTENT_TYPE, "image/jpeg".parse().unwrap());

        (headers, result).into_response()
    } else {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}
