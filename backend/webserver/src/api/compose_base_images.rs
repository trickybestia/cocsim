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
use bytes::Bytes;
use image::{
    ImageReader,
    codecs::jpeg::JpegEncoder,
};

fn compose_base_images_internal(left: Vec<Bytes>, right: Vec<Bytes>) -> anyhow::Result<Bytes> {
    let mut left_images = Vec::new();

    for image in left {
        left_images.push(
            ImageReader::new(Cursor::new(image))
                .with_guessed_format()
                .expect("Cursor io never fails")
                .decode()?
                .to_rgb8(),
        );
    }

    let mut right_images = Vec::new();

    for image in right {
        right_images.push(
            ImageReader::new(Cursor::new(image))
                .with_guessed_format()
                .expect("Cursor io never fails")
                .decode()?
                .to_rgb8(),
        );
    }

    let result = compose_base_images::compose_base_images(&left_images, &right_images);
    let mut result_writer = Cursor::new(Vec::new());

    let mut encoder = JpegEncoder::new_with_quality(&mut result_writer, 80);

    encoder.encode_image(&result)?;

    Ok(Bytes::from_owner(result_writer.into_inner()))
}

pub async fn compose_base_images(mut multipart: Multipart) -> impl IntoResponse {
    let mut left = Vec::new();
    let mut right = Vec::new();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        if name == "left" {
            left.push(data);
        } else if name == "right" {
            right.push(data);
        }
    }

    if let Ok(result) = compose_base_images_internal(left, right) {
        let mut headers = HeaderMap::new();
        headers.insert(header::CONTENT_TYPE, "image/jpeg".parse().unwrap());

        (headers, result).into_response()
    } else {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}
