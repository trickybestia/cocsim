// https://github.com/kosinix/raster

use std::cmp;

use image::RgbImage;

/// Rotate an image clockwise. Negate the radians to do a counter-clockwise
/// rotation. Background color is black.
///
/// Note: If you look closely, the quality for arbitrary angles is not very good
/// due to the simple sampling algorithm. The 90, 180, and 270 degrees angles
/// looks fine because no pixels are lost. This will be fixed in the future with
/// a better sampling algorithm.
pub fn rotate(src: &RgbImage, radians: f32) -> RgbImage {
    let w1 = src.width() as i32;
    let h1 = src.height() as i32;

    // Using screen coords system, top left is always at (0,0)
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;

    let top_right_1: (i32, i32) = (w1, 0);
    let top_right_2: (i32, i32) = rotate_point(top_right_1, radians);
    min_x = cmp::min(min_x, top_right_2.0);
    max_x = cmp::max(max_x, top_right_2.0);
    min_y = cmp::min(min_y, top_right_2.1);
    max_y = cmp::max(max_y, top_right_2.1);

    let bottom_right_1: (i32, i32) = (w1, h1);
    let bottom_right_2: (i32, i32) = rotate_point(bottom_right_1, radians);
    min_x = cmp::min(min_x, bottom_right_2.0);
    max_x = cmp::max(max_x, bottom_right_2.0);
    min_y = cmp::min(min_y, bottom_right_2.1);
    max_y = cmp::max(max_y, bottom_right_2.1);

    let bottom_left_1: (i32, i32) = (0, h1);
    let bottom_left_2: (i32, i32) = rotate_point(bottom_left_1, radians);
    min_x = cmp::min(min_x, bottom_left_2.0);
    max_x = cmp::max(max_x, bottom_left_2.0);
    min_y = cmp::min(min_y, bottom_left_2.1);
    max_y = cmp::max(max_y, bottom_left_2.1);

    let w2 = ((min_x as f32).abs() + (max_x as f32).abs()) as i32 + 1;
    let h2 = ((min_y as f32).abs() + (max_y as f32).abs()) as i32 + 1;
    let mut dest = RgbImage::new(w2 as u32, h2 as u32);

    for (dest_y, y) in (0..).zip(min_y..max_y + 1) {
        for (dest_x, x) in (0..).zip(min_x..max_x + 1) {
            let point: (i32, i32) = rotate_point((x, y), -radians);

            if point.0 >= 0 && point.0 < w1 && point.1 >= 0 && point.1 < h1 {
                let pixel = src.get_pixel(point.0 as u32, point.1 as u32);
                dest.put_pixel(dest_x, dest_y, *pixel);
            }
        }
    }

    dest
}

/// Rotate a point clockwise to a given degree.
fn rotate_point(p: (i32, i32), radians: f32) -> (i32, i32) {
    let px: f32 = p.0 as f32;
    let py: f32 = p.1 as f32;
    let cos = radians.cos();
    let sin = radians.sin();
    let x = ((px * cos) - (py * sin)).round();
    let y = ((px * sin) + (py * cos)).round();
    (x as i32, y as i32)
}
