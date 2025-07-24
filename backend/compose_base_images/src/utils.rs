use std::path::Path;

use anyhow::Context;
use image::{
    ImageReader,
    RgbImage,
};
use ndarray::{
    ArrayBase,
    ArrayView3,
    ArrayViewMut3,
    Axis,
    Data,
    DataMut,
    Ix3,
    s,
};

pub fn load_base_images(path: &Path) -> anyhow::Result<(Vec<RgbImage>, Vec<RgbImage>)> {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for file in path.read_dir()? {
        let file = file?;
        let file_name = file.file_name();
        let file_name = file_name
            .to_str()
            .context("Expected normal image file name")?;

        if file_name.starts_with("l") {
            left.push(ImageReader::open(file.path())?.decode()?.into_rgb8());
        } else if file_name.starts_with("r") {
            right.push(ImageReader::open(file.path())?.decode()?.into_rgb8());
        }
    }

    Ok((left, right))
}

pub fn crop<'a, S>(
    image: &'a ArrayBase<S, Ix3>,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> ArrayView3<'a, u8>
where
    S: Data<Elem = u8>,
{
    image.slice(s![.., y..(y + height), x..(x + width)])
}

pub fn crop_mut<'a, S>(
    image: &'a mut ArrayBase<S, Ix3>,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> ArrayViewMut3<'a, u8>
where
    S: DataMut<Elem = u8>,
{
    image.slice_mut(s![.., y..(y + height), x..(x + width)])
}

pub fn width<S>(image: &ArrayBase<S, Ix3>) -> usize
where
    S: Data<Elem = u8>,
{
    image.len_of(Axis(2))
}

pub fn height<S>(image: &ArrayBase<S, Ix3>) -> usize
where
    S: Data<Elem = u8>,
{
    image.len_of(Axis(1))
}
