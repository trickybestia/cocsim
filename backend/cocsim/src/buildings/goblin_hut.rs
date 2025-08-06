use arbitrary::Arbitrary;
use nalgebra::Vector2;
use serde::{
    Deserialize,
    Serialize,
};
use hecs::World;

use crate::{
    BuildingModel,
    BuildingType,
    UsizeWithMax,
    buildings::utils::passive_building::create_passive_building,
    consts::MAX_BUILDING_POS,
};

struct GoblinHutLevel {
    pub health: f32,
}

const GOBLIN_HUT_LEVELS_LEN: usize = 1;
const GOBLIN_HUT_LEVEL_INDEX_MAX: usize = GOBLIN_HUT_LEVELS_LEN - 1;
const GOBLIN_HUT_LEVELS: [GoblinHutLevel; GOBLIN_HUT_LEVELS_LEN] =
    [GoblinHutLevel { health: 250.0 }];

const GOBLIN_HUT: BuildingType = BuildingType {
    name: "GoblinHut",
    size: Vector2::new(2, 2),
    levels: GOBLIN_HUT_LEVELS.len(),
    options: &[],
};

inventory::submit! {GOBLIN_HUT}

#[derive(Serialize, Deserialize, Debug, Arbitrary, Clone)]
pub struct GoblinHutModel {
    pub x: UsizeWithMax<MAX_BUILDING_POS>,
    pub y: UsizeWithMax<MAX_BUILDING_POS>,
    pub level: UsizeWithMax<GOBLIN_HUT_LEVEL_INDEX_MAX>,
}

impl BuildingModel for GoblinHutModel {
    fn r#type(&self) -> &'static BuildingType {
        &GOBLIN_HUT
    }

    fn position(&self) -> Vector2<usize> {
        Vector2::new(*self.x, *self.y)
    }

    fn spawn(&self, world: &mut World) {
        create_passive_building(
            world,
            GOBLIN_HUT_LEVELS[*self.level].health,
            Vector2::new(*self.x, *self.y),
            GOBLIN_HUT.size,
        );
    }
}
