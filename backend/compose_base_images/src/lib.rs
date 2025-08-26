mod raster_stolen_functions;
pub mod utils;

use std::{
    cmp::min,
    f32::consts::FRAC_PI_4,
};

use image::{
    RgbImage,
    imageops::{
        FilterType,
        resize,
    },
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
use nshare::{
    AsNdarray3,
    AsNdarray3Mut,
};

use crate::{
    raster_stolen_functions::rotate,
    utils::{
        crop,
        crop_mut,
        height,
        width,
    },
};

const VIGNETTE_STRENGTH: f32 = 0.26;

pub fn reverse_projection(image: &RgbImage) -> RgbImage {
    let resized = resize(
        image,
        image.width(),
        image.height() * 4 / 3,
        FilterType::Triangle,
    );

    rotate(&resized, FRAC_PI_4)
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

    for left_image in left.iter().skip(1) {
        left_composed = compose_vertical_base_images(
            &left_composed,
            left_image,
            VERTICAL_IGNORE_BORDERS,
            Y_SKIP_FIRST,
            Y_SKIP_LAST,
        );
    }

    let mut right_composed = right[0].to_owned();

    for right_image in right.iter().skip(1) {
        right_composed = compose_vertical_base_images(
            &right_composed,
            right_image,
            VERTICAL_IGNORE_BORDERS,
            Y_SKIP_FIRST,
            Y_SKIP_LAST,
        );
    }

    let min_height = min(height(&left_composed), height(&right_composed));

    let mut left_composed = crop(&left_composed, 0, 0, width(&left_composed), min_height);

    const HORIZONTAL_IGNORE_BORDERS: usize = 150;
    const X_SKIP_FIRST: usize = 400;
    const X_SKIP_LAST: usize = 400;

    let mut right_composed = crop(
        &right_composed,
        width(&right_composed) / 2,
        0,
        width(&right_composed) - width(&right_composed) / 2,
        min_height,
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

    let max = *vignette.iter().max_by(|a, b| a.total_cmp(b)).unwrap();

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

    let mut composed = Array3::zeros((3, bottom_paste_y + height(bottom), width(top)));

    crop_mut(&mut composed, 0, 0, width(top), height(top)).assign(top);
    crop_mut(
        &mut composed,
        0,
        bottom_paste_y,
        width(bottom),
        height(bottom),
    )
    .assign(bottom);

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
    assert!(width(top) == width(bottom));

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
