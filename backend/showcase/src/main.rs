mod consts;

use cocsim::{
    Game,
    Shape,
    utils::load_test_map,
};
use consts::*;
use macroquad::prelude::*;

fn draw_shapes(shapes: &[Shape]) {
    for shape in shapes {
        match shape {
            Shape::Rect {
                x,
                y,
                width,
                height,
                color,
            } => {
                draw_rectangle(
                    x * PIXELS_PER_TILE as f32,
                    y * PIXELS_PER_TILE as f32,
                    width * PIXELS_PER_TILE as f32,
                    height * PIXELS_PER_TILE as f32,
                    Color::from_rgba(color.r, color.g, color.b, 255),
                );
            }
            Shape::Circle {
                x,
                y,
                radius,
                color,
            } => {
                draw_circle(
                    x * PIXELS_PER_TILE as f32,
                    y * PIXELS_PER_TILE as f32,
                    radius * PIXELS_PER_TILE as f32,
                    Color::from_rgba(color.r, color.g, color.b, 255),
                );
            }
            Shape::Line {
                x1,
                y1,
                x2,
                y2,
                width,
                color,
            } => {
                draw_line(
                    x1 * PIXELS_PER_TILE as f32,
                    y1 * PIXELS_PER_TILE as f32,
                    x2 * PIXELS_PER_TILE as f32,
                    y2 * PIXELS_PER_TILE as f32,
                    width * PIXELS_PER_TILE as f32,
                    Color::from_rgba(color.r, color.g, color.b, 255),
                );
            }
        }
    }
}

#[macroquad::main("cocsim")]
async fn main() {
    let (map, map_image) = load_test_map("single_player/goblin_gauntlet").unwrap();

    let mut game = Game::new(&map).unwrap();

    let grid = game.draw_grid();
    let mut collision = game.draw_collision();

    loop {
        if game.need_redraw_collision() {
            collision = game.draw_collision();
        }

        let entities = game.draw_entities();

        clear_background(BLACK);

        draw_shapes(&grid);
        draw_shapes(&collision);
        draw_shapes(&entities);

        if !game.done() {
            game.tick(get_frame_time());
        }

        next_frame().await
    }
}
