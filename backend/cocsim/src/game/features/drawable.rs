use hecs::{
    Entity,
    PreparedQuery,
    World,
};

use crate::{
    Game,
    Shape,
    ShapeColor,
    game::features::position::Position,
    utils::AnyMapExt,
};

pub struct Drawable {
    pub draw_fn: fn(Entity, &World, &mut Vec<Shape>),
}

pub struct Line {
    pub a: Entity,
    pub b: Entity,
    pub width: f32,
    pub color: ShapeColor,
}

pub fn draw(result: &mut Vec<Shape>, game: &mut Game) {
    for (id, drawable) in game
        .cache
        .get_mut_or_default::<PreparedQuery<&Drawable>>()
        .query(&game.world)
        .iter()
    {
        (drawable.draw_fn)(id, &game.world, result);
    }

    for (_, line) in game
        .cache
        .get_mut_or_default::<PreparedQuery<&Line>>()
        .query(&game.world)
        .iter()
    {
        if let Ok(a_position) = game.world.get::<&Position>(line.a)
            && let Ok(b_position) = game.world.get::<&Position>(line.b)
        {
            result.push(Shape::Line {
                x1: a_position.0.x,
                y1: a_position.0.y,
                x2: b_position.0.x,
                y2: b_position.0.y,
                width: line.width,
                color: line.color,
            });
        }
    }
}
