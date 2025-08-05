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

struct LaboratoryLevel {
    pub health: f32,
}

const LABORATORY_LEVELS_LEN: usize = 14;
const LABORATORY_LEVEL_INDEX_MAX: usize = LABORATORY_LEVELS_LEN - 1;
const LABORATORY_LEVELS: [LaboratoryLevel; LABORATORY_LEVELS_LEN] = [
    LaboratoryLevel { health: 500.0 },
    LaboratoryLevel { health: 550.0 },
    LaboratoryLevel { health: 600.0 },
    LaboratoryLevel { health: 650.0 },
    LaboratoryLevel { health: 700.0 },
    LaboratoryLevel { health: 750.0 },
    LaboratoryLevel { health: 830.0 },
    LaboratoryLevel { health: 950.0 },
    LaboratoryLevel { health: 1070.0 },
    LaboratoryLevel { health: 1140.0 },
    LaboratoryLevel { health: 1210.0 },
    LaboratoryLevel { health: 1280.0 },
    LaboratoryLevel { health: 1350.0 },
    LaboratoryLevel { health: 1400.0 },
];

const LABORATORY: BuildingType = BuildingType {
    name: "Laboratory",
    size: Vector2::new(3, 3),
    levels: LABORATORY_LEVELS.len(),
    options: &[],
};

inventory::submit! {LABORATORY}

#[derive(Serialize, Deserialize, Debug, Arbitrary, Clone)]
pub struct LaboratoryModel {
    pub x: UsizeWithMax<MAX_BUILDING_POS>,
    pub y: UsizeWithMax<MAX_BUILDING_POS>,
    pub level: UsizeWithMax<LABORATORY_LEVEL_INDEX_MAX>,
}

impl BuildingModel for LaboratoryModel {
    fn r#type(&self) -> &'static BuildingType {
        &LABORATORY
    }

    fn position(&self) -> Vector2<usize> {
        Vector2::new(*self.x, *self.y)
    }

    fn create_building(&self, world: &mut World) {
        create_passive_building(
            world,
            LABORATORY_LEVELS[*self.level].health,
            Vector2::new(*self.x, *self.y),
            LABORATORY.size,
        );
    }
}
