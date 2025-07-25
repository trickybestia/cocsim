mod utils;

use std::{
    cmp::min,
    io::Cursor,
};

use anyhow::ensure;
use image::{
    ImageFormat,
    ImageReader,
    RgbImage,
};
use magick_rust::{
    MagickWand,
    bindings::{
        DistortMethod_AffineDistortion,
        MagickBooleanType_MagickFalse,
        MagickBooleanType_MagickTrue,
        MagickDistortImage,
    },
    magick_wand_genesis,
};
use ndarray::{
    Array2,
    Array3,
    ArrayBase,
    Axis,
    Data,
    Ix3,
    s,
};
use ndarray_stats::QuantileExt;
use nshare::{
    AsNdarray3,
    AsNdarray3Mut,
};
pub use utils::load_base_images;

use crate::utils::{
    crop,
    crop_mut,
    height,
    width,
};

const VIGNETTE_STRENGTH: f32 = 0.26;

pub fn reverse_projection<T: AsRef<[u8]>>(image: T) -> anyhow::Result<RgbImage> {
    const RESIZE_WIDTH: usize = 2498;
    const RESIZE_HEIGHT: usize = 1756;
    const RESIZE_ASPECT_RATIO: f32 = RESIZE_WIDTH as f32 / RESIZE_HEIGHT as f32;

    magick_wand_genesis();

    let mut wand = MagickWand::new();

    wand.read_image_blob(image)?;

    let aspect_ratio = wand.get_image_width() as f32 / wand.get_image_height() as f32;

    if aspect_ratio > RESIZE_ASPECT_RATIO {
        wand.crop_image(
            (wand.get_image_height() as f32 * RESIZE_ASPECT_RATIO).round() as usize,
            wand.get_image_height(),
            0,
            0,
        )?;
    } else {
        wand.crop_image(
            wand.get_image_width(),
            (wand.get_image_width() as f32 / RESIZE_ASPECT_RATIO).round() as usize,
            0,
            0,
        )?;
    }

    const TOP_CORNER_POS: (f64, f64) = (1250.0, 41.0);
    const BOTTOM_CORNER_POS: (f64, f64) = (1247.0, 1572.0);
    const LEFT_CORNER_POS: (f64, f64) = (223.0, 810.0);

    wand.set_image_artifact("distort:viewport", "1800x1800+0+0")?;

    let args = [
        TOP_CORNER_POS.0,
        TOP_CORNER_POS.1,
        1500.0,
        300.0,
        BOTTOM_CORNER_POS.0,
        BOTTOM_CORNER_POS.1,
        300.0,
        1500.0,
        LEFT_CORNER_POS.0,
        LEFT_CORNER_POS.1,
        300.0,
        300.0,
    ];

    let ok;

    unsafe {
        ok = MagickDistortImage(
            wand.wand,
            DistortMethod_AffineDistortion,
            args.len(),
            args.as_ptr(),
            MagickBooleanType_MagickFalse,
        );
    }

    ensure!(
        ok == MagickBooleanType_MagickTrue,
        "MagickBooleanType_MagickTrue expected"
    );

    Ok(
        ImageReader::with_format(Cursor::new(wand.write_image_blob("BMP")?), ImageFormat::Bmp)
            .decode()?
            .to_rgb8(),
    )
}

pub fn compose_base_images(left: &[RgbImage], right: &[RgbImage]) -> RgbImage {
    let left = left
        .iter()
        .map(|image| remove_vignette(&image.as_ndarray3(), VIGNETTE_STRENGTH))
        .collect::<Vec<_>>();
    let right = right
        .iter()
        .map(|image| remove_vignette(&image.as_ndarray3(), VIGNETTE_STRENGTH))
        .collect::<Vec<_>>();

    const VERTICAL_IGNORE_BORDERS: usize = 400;
    const Y_SKIP_FIRST: usize = 200;
    const Y_SKIP_LAST: usize = 200;

    let mut left_composed = left[0].to_owned();

    for i in 1..left.len() {
        left_composed = compose_vertical_base_images(
            &left_composed,
            &left[i],
            VERTICAL_IGNORE_BORDERS,
            Y_SKIP_FIRST,
            Y_SKIP_LAST,
        );
    }

    let mut right_composed = right[0].to_owned();

    for i in 1..right.len() {
        right_composed = compose_vertical_base_images(
            &right_composed,
            &right[i],
            VERTICAL_IGNORE_BORDERS,
            Y_SKIP_FIRST,
            Y_SKIP_LAST,
        );
    }

    const HORIZONTAL_IGNORE_BORDERS: usize = 150;
    const X_SKIP_FIRST: usize = 400;
    const X_SKIP_LAST: usize = 400;

    let mut right_composed = crop(
        &right_composed,
        width(&right_composed) / 2,
        0,
        width(&right_composed) - width(&right_composed) / 2,
        height(&right_composed),
    );

    right_composed.swap_axes(1, 2);
    left_composed.swap_axes(1, 2);

    let mut result = compose_vertical_base_images(
        &left_composed,
        &right_composed,
        HORIZONTAL_IGNORE_BORDERS,
        X_SKIP_FIRST,
        X_SKIP_LAST,
    );

    result.swap_axes(1, 2);

    let mut result_image = RgbImage::new(width(&result) as u32, height(&result) as u32);

    AsNdarray3Mut::as_ndarray3_mut(&mut result_image).assign(&result);

    result_image
}

fn remove_vignette<S>(image: &ArrayBase<S, Ix3>, strength: f32) -> Array3<u8>
where
    S: Data<Elem = u8>,
{
    let width = width(image);
    let height = height(image);

    let vignette = Array2::from_shape_fn((height, width), |(y, x)| {
        let x = x as f32 / width as f32 * 2.0 - 1.0;
        let y = y as f32 / height as f32 * 2.0 - 1.0;

        1.0 - strength * (x.powi(2) + y.powi(2))
    });

    let max = *vignette.max().unwrap();

    let mut image = image.to_owned();

    for channel in 0..image.len_of(Axis(0)) {
        image
            .slice_mut(s![channel, .., ..])
            .zip_mut_with(&vignette, |pixel, vignette_strength| {
                *pixel = (*pixel as f32 * max / vignette_strength) as u8;
            });
    }

    image
}

fn compose_vertical_base_images<S1, S2>(
    top: &ArrayBase<S1, Ix3>,
    bottom: &ArrayBase<S2, Ix3>,
    ignore_borders: usize,
    y_skip_first: usize,
    y_skip_last: usize,
) -> Array3<u8>
where
    S1: Data<Elem = u8>,
    S2: Data<Elem = u8>,
{
    let bottom_paste_y = find_tear_line_y(
        top,
        bottom,
        ignore_borders,
        y_skip_first,
        height(top) - y_skip_last,
    );

    let mut composed = Array3::zeros((3, bottom_paste_y + height(&bottom), width(top)));

    crop_mut(&mut composed, 0, 0, width(&top), height(&top)).assign(&top);
    crop_mut(
        &mut composed,
        0,
        bottom_paste_y,
        width(&bottom),
        height(&bottom),
    )
    .assign(&bottom);

    composed
}

fn find_tear_line_y<S1, S2>(
    top: &ArrayBase<S1, Ix3>,
    bottom: &ArrayBase<S2, Ix3>,
    ignore_borders: usize,
    y_start: usize,
    y_stop: usize,
) -> usize
where
    S1: Data<Elem = u8>,
    S2: Data<Elem = u8>,
{
    const WINDOW_HEIGHT: usize = 100;

    let top = crop(
        top,
        ignore_borders,
        0,
        width(top) - 2 * ignore_borders,
        height(top),
    );
    let bottom = crop(
        bottom,
        ignore_borders,
        0,
        width(bottom) - 2 * ignore_borders,
        height(bottom),
    );

    let mut smallest_difference = None;
    let mut smallest_difference_y = None;

    for y in y_start..min(y_stop, height(&top) - WINDOW_HEIGHT) {
        let top_crop = crop(&top, 0, y, width(&top), WINDOW_HEIGHT);
        let bottom_crop = crop(&bottom, 0, 0, width(&bottom), WINDOW_HEIGHT);
        let difference = (top_crop.mapv(|elem| elem as i32) - bottom_crop.mapv(|elem| elem as i32))
            .mapv_into(i32::abs)
            .sum();

        if smallest_difference.is_none_or(|smallest_difference| difference < smallest_difference) {
            smallest_difference_y = Some(y);
            smallest_difference = Some(difference);
        }
    }

    smallest_difference_y.unwrap()
}
