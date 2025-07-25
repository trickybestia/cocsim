use std::{
    env::args,
    io::Cursor,
    path::PathBuf,
};

use compose_base_images::{
    compose_base_images,
    load_base_images,
    reverse_projection,
};
use image::codecs::bmp::BmpEncoder;

fn main() {
    let images_dir_path = args().nth(1).unwrap().parse::<PathBuf>().unwrap();

    let (left, right) = load_base_images(&images_dir_path).unwrap();

    let composed = compose_base_images(&left, &right);

    composed.save(images_dir_path.join("composed.png")).unwrap();

    let mut buffer = Cursor::new(Vec::new());
    let encoder = BmpEncoder::new(&mut buffer);

    composed.write_with_encoder(encoder).unwrap();

    let reversed = reverse_projection(buffer.into_inner()).unwrap();

    reversed.save(images_dir_path.join("reversed.png")).unwrap();
}
