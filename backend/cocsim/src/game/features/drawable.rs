use hecs::Entity;

use crate::{
    Game,
    Shape,
};

pub struct Drawable {
    pub draw_fn: fn(Entity, &Game, &mut Vec<Shape>),
}

pub fn draw(result: &mut Vec<Shape>, game: &Game) {
    for (id, drawable) in game.world.query::<&Drawable>().iter() {
        (drawable.draw_fn)(id, game, result);
    }
}
