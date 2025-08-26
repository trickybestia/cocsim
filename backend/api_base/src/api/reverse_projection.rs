use std::io::Cursor;

use bytes::Bytes;
use image::{
    ImageReader,
    codecs::jpeg::JpegEncoder,
};

pub fn reverse_projection(image_bytes: Bytes) -> anyhow::Result<Vec<u8>> {
    let image = ImageReader::new(Cursor::new(image_bytes))
        .with_guessed_format()
        .expect("Cursor io never fails")
        .decode()?
        .to_rgb8();

    let result = compose_base_images::reverse_projection(&image);

    let mut result_writer = Cursor::new(Vec::new());

    let mut encoder = JpegEncoder::new_with_quality(&mut result_writer, 70);

    encoder.encode_image(&result)?;

    Ok(result_writer.into_inner())
}
