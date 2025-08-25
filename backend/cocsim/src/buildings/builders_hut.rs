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

struct BuildersHutLevel {
    pub health: f32,
}

const BUILDERS_HUT_LEVELS_LEN: usize = 7;
const BUILDERS_HUT_LEVEL_INDEX_MAX: usize = BUILDERS_HUT_LEVELS_LEN - 1;
const BUILDERS_HUT_LEVELS: [BuildersHutLevel; BUILDERS_HUT_LEVELS_LEN] = [
    BuildersHutLevel { health: 250.0 },
    BuildersHutLevel { health: 1000.0 },
    BuildersHutLevel { health: 1300.0 },
    BuildersHutLevel { health: 1600.0 },
    BuildersHutLevel { health: 1800.0 },
    BuildersHutLevel { health: 1900.0 },
    BuildersHutLevel { health: 2000.0 },
];

const BUILDERS_HUT: BuildingType = BuildingType {
    name: "BuildersHut",
    size: Vector2::new(2, 2),
    levels: BUILDERS_HUT_LEVELS.len(),
    options: &[],
    affects_drop_zone: true,
};

inventory::submit! {BUILDERS_HUT}

#[derive(Serialize, Deserialize, Debug, Arbitrary, Clone)]
pub struct BuildersHutModel {
    pub level: UsizeWithMax<BUILDERS_HUT_LEVEL_INDEX_MAX>,
}

impl BuildingModel for BuildersHutModel {
    fn r#type(&self) -> &'static BuildingType {
        &BUILDERS_HUT
    }

    fn spawn(&self, world: &mut World, position: Vector2<usize>) {
        spawn_other_building(
            world,
            BUILDERS_HUT_LEVELS[*self.level].health,
            position,
            BUILDERS_HUT.size,
        );
    }
}
