use cocsim::{
    Game,
    Shape,
};
use macroquad::{
    color::{
        BLACK,
        Color,
        WHITE,
    },
    math::Vec2,
    shapes::{
        draw_circle,
        draw_line,
        draw_rectangle,
    },
    texture::{
        DrawTextureParams,
        Texture2D,
        draw_texture_ex,
    },
    time::get_frame_time,
    window::{
        clear_background,
        next_frame,
    },
};

use crate::consts::PIXELS_PER_TILE;

pub fn draw_shapes(shapes: &[Shape], alpha: u8) {
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

pub async fn macroquad_run_game(
    game: &mut Game,
    map_image: &[u8],
    mut before_tick: Option<Box<dyn FnMut(&mut Game)>>,
) {
    let map_texture = Texture2D::from_file_with_format(&map_image, None);
    let map_texture_size = PIXELS_PER_TILE * game.map_size().total_size() as usize;

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
        draw_shapes(&collision, 100);
        draw_shapes(&entities, 255);

        if !game.done() {
            if let Some(before_tick) = &mut before_tick {
                before_tick(game);
            }

            game.tick(get_frame_time());
        }

        next_frame().await
    }
}
