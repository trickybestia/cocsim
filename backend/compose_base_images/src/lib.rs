mod utils;

use std::cmp::min;

use image::RgbImage;
use ndarray::{
    Array3,
    ArrayBase,
    Data,
    Ix3,
};
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

pub fn compose_base_images(left: &[RgbImage], right: &[RgbImage]) -> RgbImage {
    let left = left.iter().map(AsNdarray3::as_ndarray3).collect::<Vec<_>>();
    let right = right
        .iter()
        .map(AsNdarray3::as_ndarray3)
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

    /*let mut right_composed = right[0].to_owned();

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

    let mut left_composed = crop(
        &left_composed,
        0,
        0,
        width(&left_composed) / 2,
        height(&left_composed),
    );

    right_composed.swap_axes(1, 2);
    left_composed.swap_axes(1, 2);

    let mut result = compose_vertical_base_images(
        &right_composed,
        &left_composed,
        HORIZONTAL_IGNORE_BORDERS,
        X_SKIP_FIRST,
        X_SKIP_LAST,
    );

    result.swap_axes(1, 2);*/

    let result = left_composed;

    let mut result_image = RgbImage::new(width(&result) as u32, height(&result) as u32);

    AsNdarray3Mut::as_ndarray3_mut(&mut result_image).assign(&result);

    result_image
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
            .sum()
            .saturating_abs();

        if smallest_difference.is_none_or(|smallest_difference| difference < smallest_difference) {
            smallest_difference_y = Some(y);
            smallest_difference = Some(difference);
        }
    }

    smallest_difference_y.unwrap()
}
