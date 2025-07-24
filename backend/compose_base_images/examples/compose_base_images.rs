use std::{
    env::args,
    path::PathBuf,
};

use compose_base_images::{
    compose_base_images,
    load_base_images,
};

fn main() {
    let images_dir_path = args().nth(1).unwrap().parse::<PathBuf>().unwrap();

    let (left, right) = load_base_images(&images_dir_path).unwrap();

    let composed = compose_base_images(&left, &right);

    composed.save(images_dir_path.join("composed.png")).unwrap();
}
