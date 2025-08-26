use std::io::Cursor;

use bytes::Bytes;
use image::{
    ImageReader,
    codecs::jpeg::JpegEncoder,
};

pub fn compose_base_images(
    left: impl IntoIterator<Item = Bytes>,
    right: impl IntoIterator<Item = Bytes>,
) -> anyhow::Result<Vec<u8>> {
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

    let mut encoder = JpegEncoder::new_with_quality(&mut result_writer, 70);

    encoder.encode_image(&result)?;

    Ok(result_writer.into_inner())
}
