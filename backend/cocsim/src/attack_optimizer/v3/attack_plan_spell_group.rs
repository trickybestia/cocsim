use nalgebra::Vector2;

use crate::{
    consts::MAX_UNIT_DROP_TIME,
    game::features::map_size::MapSize,
};

#[derive(Clone, Debug)]
pub struct AttackPlanSpellGroup {
    /// 0.0 <= each component <= 1.0.
    pub position: Vector2<f32>,
    pub drop_time: f32,
}

impl AttackPlanSpellGroup {
    pub fn from_numbers(x: &mut impl Iterator<Item = f32>) -> Self {
        Self {
            position: Vector2::new(x.next().unwrap(), x.next().unwrap()),
            drop_time: x.next().unwrap(),
        }
    }

    pub fn domain() -> impl Iterator<Item = (f32, f32)> {
        [
            (0.0, 1.0),                // position.x
            (0.0, 1.0),                // position.y
            (0.0, MAX_UNIT_DROP_TIME), // drop_time
        ]
        .into_iter()
    }

    pub fn cartesian_position(&self, map_size: &MapSize) -> Vector2<f32> {
        self.position * map_size.total_size() as f32
    }
}
