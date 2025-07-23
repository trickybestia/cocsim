mod consts;

use cocsim::{
    DragonModel,
    Game,
    Shape,
    utils::load_test_map,
};
use consts::*;
use macroquad::prelude::*;
use nalgebra::Vector2;

fn draw_shapes(shapes: &[Shape], alpha: u8) {
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
                    Color::from_rgba(color.r, color.g, color.b, alpha),
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
                    Color::from_rgba(color.r, color.g, color.b, alpha),
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
                    Color::from_rgba(color.r, color.g, color.b, alpha),
                );
            }
        }
    }
}

#[macroquad::main("cocsim")]
async fn main() {
    let (map, map_image) = load_test_map("single_player/goblin_gauntlet").unwrap();

    let map_texture = Texture2D::from_file_with_format(&map_image, None);
    let map_texture_size = PIXELS_PER_TILE * (map.base_size + 2 * map.border_size);

    let mut game = Game::new(&map).unwrap();

    for i in 0..10 {
        game.spawn_unit(
            &DragonModel { level: 10 }.into(),
            Vector2::new(0.5, i as f32 + 0.5),
        )
        .unwrap();
    }

    let grid = game.draw_grid();
    let mut collision = game.draw_collision();

    loop {
        if game.need_redraw_collision() {
            collision = game.draw_collision();
        }

        let entities = game.draw_entities();

        clear_background(BLACK);

        draw_texture_ex(
            &map_texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(map_texture_size as f32, map_texture_size as f32)),
                ..Default::default()
            },
        );

        draw_shapes(&grid, 100);
        draw_shapes(&collision, 255);
        draw_shapes(&entities, 255);

        if !game.done() {
            game.tick(get_frame_time());
        }

        next_frame().await
    }
}
