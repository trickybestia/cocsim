use cocsim::utils::nearest_point_on_arc;
use macroquad::{
    color::Color,
    color_u8,
    input::{
        MouseButton,
        is_mouse_button_down,
        mouse_position,
    },
    shapes::{
        draw_arc,
        draw_circle,
    },
    window::next_frame,
};
use nalgebra::Vector2;

#[macroquad::main("nearest point on arc")]
async fn main() {
    let arc_radius = 60.0;
    let arc_start = 45.0;
    let arc_angle = 90.0;
    let mut arc_center = Vector2::new(100.0, 100.0);
    let mut point = Vector2::new(10.0, 10.0);

    loop {
        if is_mouse_button_down(MouseButton::Right) {
            let (mouse_x, mouse_y) = mouse_position();

            arc_center = Vector2::new(mouse_x, mouse_y);
        }

        if is_mouse_button_down(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();

            point = Vector2::new(mouse_x, mouse_y);
        }

        let nearest_point =
            nearest_point_on_arc(point, arc_center, arc_radius, arc_start, arc_angle);

        dbg!(nearest_point.metric_distance(&point));

        draw_circle(arc_center.x, arc_center.y, 6.0, color_u8!(255, 0, 0, 255));
        draw_arc(
            arc_center.x,
            arc_center.y,
            100,
            arc_radius,
            arc_start,
            6.0,
            arc_angle,
            color_u8!(255, 0, 0, 255),
        );

        draw_circle(point.x, point.y, 6.0, color_u8!(255, 255, 255, 255));

        draw_circle(
            nearest_point.x,
            nearest_point.y,
            6.0,
            color_u8!(0, 255, 0, 255),
        );

        next_frame().await;
    }
}
