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

pub async fn compose_base_images(mut multipart: Multipart) -> Result<Response, WebserverError> {
    let mut left = Vec::new();
    let mut right = Vec::new();

    while let Some(field) = multipart.next_field().await? {
        let name = field
            .name()
            .context("Field name not found I guess")?
            .to_string();
        let data = field.bytes().await?;

        if name == "left" {
            left.push(data);
        } else if name == "right" {
            right.push(data);
        }
    }

    let result = spawn_blocking(|| api_base::compose_base_images(left, right)).await??;

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        "image/jpeg".parse().expect("Should be valid content type"),
    );

    Ok((headers, result).into_response())
}
