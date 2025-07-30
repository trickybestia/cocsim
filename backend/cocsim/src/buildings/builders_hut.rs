use arbitrary::Arbitrary;
use nalgebra::Vector2;
use serde::{
    Deserialize,
    Serialize,
};
use shipyard::World;

use crate::{
    BuildingModel,
    BuildingType,
    UsizeWithMax,
    buildings::utils::passive_building::create_passive_building,
    consts::MAX_BUILDING_POS,
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
};

inventory::submit! {BUILDERS_HUT}

#[derive(Serialize, Deserialize, Debug, Arbitrary)]
pub struct BuildersHutModel {
    pub x: UsizeWithMax<MAX_BUILDING_POS>,
    pub y: UsizeWithMax<MAX_BUILDING_POS>,
    pub level: UsizeWithMax<BUILDERS_HUT_LEVEL_INDEX_MAX>,
}

impl BuildingModel for BuildersHutModel {
    fn r#type(&self) -> &'static BuildingType {
        &BUILDERS_HUT
    }

    fn position(&self) -> Vector2<usize> {
        Vector2::new(*self.x, *self.y)
    }

    fn create_building(&self, world: &mut World) {
        create_passive_building(
            world,
            BUILDERS_HUT_LEVELS[*self.level].health,
            Vector2::new(*self.x, *self.y),
            BUILDERS_HUT.size,
        );
    }
}
