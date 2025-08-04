use std::{
    collections::HashSet,
    env,
    fs::File,
    hash::Hash,
    io::{
        BufReader,
        Read,
    },
    path::PathBuf,
};

use nalgebra::{
    DMatrix,
    Vector2,
};
use zip::ZipArchive;

use crate::{
    Map,
    Shape,
    ShapeColor,
    consts::*,
};

pub fn load_test_map_raw(name: &str) -> anyhow::Result<(String, Vec<u8>)> {
    let path = PathBuf::from(env::var("TEST_MAPS_PATH")?)
        .join(name)
        .with_extension("zip");
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut archive = ZipArchive::new(reader)?;

    let mut map_image = Vec::new();
    let mut map_json = String::new();

    archive.by_name("map.json")?.read_to_string(&mut map_json)?;
    archive.by_name("map.jpg")?.read_to_end(&mut map_image)?;

    Ok((map_json, map_image))
}

pub fn load_test_map(name: &str) -> anyhow::Result<(Map, Vec<u8>)> {
    let (map_json, map_image) = load_test_map_raw(name)?;

    Ok((serde_json::from_str(&map_json)?, map_image))
}

pub fn get_tile_color(even: bool, border: bool, drop_zone: bool, occupied: bool) -> ShapeColor {
    if occupied {
        if even {
            BUILDING_TILE_EVEN_COLOR
        } else {
            BUILDING_TILE_ODD_COLOR
        }
    } else if drop_zone {
        if border {
            if even {
                DROP_ZONE_BORDER_TILE_EVEN_COLOR
            } else {
                DROP_ZONE_BORDER_TILE_ODD_COLOR
            }
        } else {
            if even {
                DROP_ZONE_TILE_EVEN_COLOR
            } else {
                DROP_ZONE_TILE_ODD_COLOR
            }
        }
    } else {
        if border {
            if even {
                BORDER_TILE_EVEN_COLOR
            } else {
                BORDER_TILE_ODD_COLOR
            }
        } else {
            if even {
                TILE_EVEN_COLOR
            } else {
                TILE_ODD_COLOR
            }
        }
    }
}

pub fn draw_bool_grid(mut grid: DMatrix<bool>, tile_size: f32, color: ShapeColor) -> Vec<Shape> {
    fn is_true_line(grid: &DMatrix<bool>, x: usize, y: usize, width: usize) -> bool {
        for x in x..(x + width) {
            if !grid[(x, y)] {
                return false;
            }
        }

        true
    }

    let mut result = Vec::new();

    for start_y in 0..grid.nrows() {
        for start_x in 0..grid.ncols() {
            if !grid[(start_x, start_y)] {
                continue;
            }

            let mut width = 1;
            let mut height = 1;

            for x in (start_x + 1)..grid.ncols() {
                if grid[(x, start_y)] {
                    width += 1;
                } else {
                    break;
                }
            }

            for y in (start_y + 1)..grid.nrows() {
                if is_true_line(&grid, start_x, y, width) {
                    height += 1;
                } else {
                    break;
                }
            }

            result.push(Shape::Rect {
                x: start_x as f32 * tile_size,
                y: start_y as f32 * tile_size,
                width: width as f32 * tile_size,
                height: height as f32 * tile_size,
                color,
            });

            for x in start_x..(start_x + width) {
                for y in start_y..(start_y + height) {
                    grid[(x, y)] = false;
                }
            }
        }
    }

    result
}

pub fn intersects<T: Hash + Eq>(a: impl Iterator<Item = T>, b: impl Iterator<Item = T>) -> bool {
    let hashset_a = a.collect::<HashSet<T>>();

    for item in b {
        if hashset_a.contains(&item) {
            return true;
        }
    }

    false
}

/// All args in degrees.
pub fn arc_contains_angle(mut arc_start: f32, arc_angle: f32, mut angle: f32) -> bool {
    if arc_angle >= 360.0 {
        return true;
    }

    arc_start = arc_start.rem_euclid(360.0);

    let arc_end = arc_start + arc_angle;

    angle = angle.rem_euclid(360.0);

    (arc_start..arc_end).contains(&angle) || (arc_start..arc_end).contains(&(angle + 360.0))
}

/// All args in degrees.
pub fn distance_on_circle(a: f32, b: f32) -> f32 {
    let result = (a.rem_euclid(360.0) - b.rem_euclid(360.0)).abs();

    if result < 180.0 {
        result
    } else {
        360.0 - result
    }
}

/// Angles in degrees.
pub fn nearest_point_on_arc(
    point: Vector2<f32>,
    arc_center: Vector2<f32>,
    arc_radius: f32,
    arc_start: f32,
    arc_angle: f32,
) -> Vector2<f32> {
    let delta = point - arc_center;

    let angle = delta.y.atan2(delta.x).to_degrees().rem_euclid(360.0);

    let angle_clipped = if arc_contains_angle(arc_start, arc_angle, angle) {
        angle
    } else {
        let arc_end = (arc_start + arc_angle).rem_euclid(360.0);

        if distance_on_circle(angle, arc_start) < distance_on_circle(angle, arc_end) {
            arc_start
        } else {
            arc_end
        }
    };

    let (sin, cos) = angle_clipped.to_radians().sin_cos();

    arc_center + Vector2::new(cos, sin) * arc_radius
}
