use arbitrary::Arbitrary;
use hecs::World;
use nalgebra::Vector2;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    UsizeWithMax,
    buildings::{
        BuildingModel,
        BuildingType,
        utils::other_building::spawn_other_building,
    },
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
    affects_drop_zone: true,
};

inventory::submit! {GOBLIN_HUT}

#[derive(Serialize, Deserialize, Debug, Arbitrary, Clone)]
pub struct GoblinHutModel {
    pub level: UsizeWithMax<GOBLIN_HUT_LEVEL_INDEX_MAX>,
}

impl BuildingModel for GoblinHutModel {
    fn r#type(&self) -> &'static BuildingType {
        &GOBLIN_HUT
    }

    fn spawn(&self, world: &mut World, position: Vector2<usize>) {
        spawn_other_building(
            world,
            GOBLIN_HUT_LEVELS[*self.level].health,
            position,
            GOBLIN_HUT.size,
        );
    }
}
