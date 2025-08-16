use arbitrary::{
    Arbitrary,
    Unstructured,
};
use nalgebra::{
    Vector2,
    clamp,
};
use rand::Rng;

use crate::{
    SpellModelEnum,
    consts::MAX_UNIT_DROP_TIME,
    game::features::map_size::MapSize,
};

#[derive(Clone, Debug)]
pub struct AttackPlanSpell {
    pub spell_model: SpellModelEnum,
    /// 0.0 <= each component <= 1.0.
    pub position: Vector2<f32>,
    pub drop_time: f32,
}

impl AttackPlanSpell {
    pub fn new_randomized(spell_model: SpellModelEnum, rng: &mut impl Rng) -> Self {
        Self {
            spell_model,
            position: Vector2::new(rng.random_range(0.0..=1.0), rng.random_range(0.0..=1.0)),
            drop_time: rng.random_range(0.0..=MAX_UNIT_DROP_TIME),
        }
    }

    pub fn mutate(&self, rng: &mut impl Rng, temperature: f32) -> Self {
        Self {
            spell_model: self.spell_model.clone(),
            position: Vector2::new(
                clamp(
                    self.position.x + rng.random_range((-0.2)..=0.2) * temperature,
                    0.0,
                    1.0,
                ),
                clamp(
                    self.position.y + rng.random_range((-0.2)..=0.2) * temperature,
                    0.0,
                    1.0,
                ),
            ),
            drop_time: clamp(
                self.drop_time + rng.random_range((-0.5)..=0.5) * temperature,
                0.0,
                MAX_UNIT_DROP_TIME,
            ),
        }
    }

    pub fn cartesian_position(&self, map_size: &MapSize) -> Vector2<f32> {
        self.position * map_size.total_size() as f32
    }
}

impl<'a> Arbitrary<'a> for AttackPlanSpell {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        Ok(Self {
            spell_model: u.arbitrary()?,
            position: Vector2::new(
                u.int_in_range::<u8>(0..=255)? as f32 / 255.0,
                u.int_in_range::<u8>(0..=255)? as f32 / 255.0,
            ),
            drop_time: (u.int_in_range::<u8>(0u8..=100u8)? as f32) / 100.0 * MAX_UNIT_DROP_TIME,
        })
    }
}
