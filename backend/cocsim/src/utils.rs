use nalgebra::DMatrix;

use crate::{
    Shape,
    ShapeColor,
    consts::*,
};

pub fn get_tile_color(even: bool, border: bool, drop_zone: bool, occupied: bool) -> &'static str {
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
                color: color.clone(),
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
