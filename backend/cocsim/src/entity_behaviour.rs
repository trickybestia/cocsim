use crate::{
    Game,
    Shape,
};

pub trait EntityBehaviour {
    fn tick(&self, game: &mut Game, id: u32, delta_t: f32);
    fn draw(&self, game: &Game, id: u32, result: &mut Vec<Shape>);
}
