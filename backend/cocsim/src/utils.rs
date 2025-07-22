use std::{
    collections::HashSet,
    fs::File,
    hash::Hash,
    io::{
        BufReader,
        Read,
    },
    path::Path,
};

use anyhow::Result;
use nalgebra::DMatrix;
use zip::ZipArchive;

use crate::{
    Map,
    Shape,
    ShapeColor,
    consts::*,
};

pub fn load_test_map_raw(name: &str) -> Result<(String, Vec<u8>)> {
    let path = Path::new(TEST_MAPS_PATH).join(name).with_extension("zip");
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut archive = ZipArchive::new(reader)?;

    let mut map_image = Vec::new();
    let mut map_json = String::new();

    archive.by_name("map.json")?.read_to_string(&mut map_json)?;
    archive.by_name("map.jpg")?.read_to_end(&mut map_image)?;

    Ok((map_json, map_image))
}

pub fn load_test_map(name: &str) -> Result<(Map, Vec<u8>)> {
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
