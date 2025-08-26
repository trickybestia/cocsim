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
use tokio::task::spawn_blocking;

use crate::webserver_error::WebserverError;

pub async fn reverse_projection(mut multipart: Multipart) -> Result<Response, WebserverError> {
    let image_field = multipart
        .next_field()
        .await?
        .context("Field name not found I guess")?;
    let image_bytes = image_field.bytes().await?;

    let result = spawn_blocking(|| api_base::reverse_projection(image_bytes)).await??;

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        "image/jpeg".parse().expect("Should be valid content type"),
    );

    Ok((headers, result).into_response())
}
