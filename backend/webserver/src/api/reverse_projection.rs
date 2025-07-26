use std::io::Cursor;

use anyhow::Context;
use axum::{
    extract::Multipart,
    http::{
        HeaderMap,
        header,
    },
    response::{
        IntoResponse,
        Response,
    },
};
use image::codecs::jpeg::JpegEncoder;

use crate::webserver_error::WebserverError;

pub async fn reverse_projection(mut multipart: Multipart) -> Result<Response, WebserverError> {
    let file = multipart
        .next_field()
        .await?
        .context("Field name not found I guess")?;
    let file_bytes = file.bytes().await?;

    let result = compose_base_images::reverse_projection(file_bytes)?;
    let mut result_writer = Cursor::new(Vec::new());

    let mut encoder = JpegEncoder::new_with_quality(&mut result_writer, 70);

    encoder.encode_image(&result)?;

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        "image/jpeg".parse().expect("Should be valid content type"),
    );

    Ok((headers, result_writer.into_inner()).into_response())
}
