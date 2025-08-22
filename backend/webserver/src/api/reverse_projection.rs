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
use bytes::Bytes;
use image::{
    ImageReader,
    codecs::jpeg::JpegEncoder,
};
use tokio::task::spawn_blocking;

use crate::webserver_error::WebserverError;

fn reverse_projection_internal(image_bytes: Bytes) -> anyhow::Result<Bytes> {
    let image = ImageReader::new(Cursor::new(image_bytes))
        .with_guessed_format()
        .expect("Cursor io never fails")
        .decode()?
        .to_rgb8();

    let result = compose_base_images::reverse_projection(&image);

    let mut result_writer = Cursor::new(Vec::new());

    let mut encoder = JpegEncoder::new_with_quality(&mut result_writer, 70);

    encoder.encode_image(&result)?;

    Ok(Bytes::from_owner(result_writer.into_inner()))
}

pub async fn reverse_projection(mut multipart: Multipart) -> Result<Response, WebserverError> {
    let field = multipart
        .next_field()
        .await?
        .context("Field name not found I guess")?;
    let field_bytes = field.bytes().await?;

    let result = spawn_blocking(|| reverse_projection_internal(field_bytes)).await??;

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        "image/jpeg".parse().expect("Should be valid content type"),
    );

    Ok((headers, result).into_response())
}
