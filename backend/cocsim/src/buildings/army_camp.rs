use arbitrary::Arbitrary;
use hecs::World;
use nalgebra::Vector2;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    BuildingModel,
    BuildingType,
    UsizeWithMax,
    buildings::utils::passive_building::create_passive_building,
    consts::MAX_BUILDING_POS,
};

struct ArmyCampLevel {
    pub health: f32,
}

const ARMY_CAMP_LEVELS_LEN: usize = 13;
const ARMY_CAMP_LEVEL_INDEX_MAX: usize = ARMY_CAMP_LEVELS_LEN - 1;
const ARMY_CAMP_LEVELS: [ArmyCampLevel; ARMY_CAMP_LEVELS_LEN] = [
    ArmyCampLevel { health: 250.0 },
    ArmyCampLevel { health: 270.0 },
    ArmyCampLevel { health: 290.0 },
    ArmyCampLevel { health: 310.0 },
    ArmyCampLevel { health: 330.0 },
    ArmyCampLevel { health: 350.0 },
    ArmyCampLevel { health: 400.0 },
    ArmyCampLevel { health: 500.0 },
    ArmyCampLevel { health: 600.0 },
    ArmyCampLevel { health: 700.0 },
    ArmyCampLevel { health: 800.0 },
    ArmyCampLevel { health: 850.0 },
    ArmyCampLevel { health: 900.0 },
];

const ARMY_CAMP: BuildingType = BuildingType {
    name: "ArmyCamp",
    size: Vector2::new(4, 4),
    levels: ARMY_CAMP_LEVELS.len(),
    options: &[],
};

inventory::submit! {ARMY_CAMP}

#[derive(Serialize, Deserialize, Debug, Arbitrary, Clone)]
pub struct ArmyCampModel {
    pub x: UsizeWithMax<MAX_BUILDING_POS>,
    pub y: UsizeWithMax<MAX_BUILDING_POS>,
    pub level: UsizeWithMax<ARMY_CAMP_LEVEL_INDEX_MAX>,
}

impl BuildingModel for ArmyCampModel {
    fn r#type(&self) -> &'static BuildingType {
        &ARMY_CAMP
    }

    fn position(&self) -> Vector2<usize> {
        Vector2::new(*self.x, *self.y)
    }

    fn spawn(&self, world: &mut World) {
        create_passive_building(
            world,
            ARMY_CAMP_LEVELS[*self.level].health,
            Vector2::new(*self.x, *self.y),
            ARMY_CAMP.size,
        );
    }
}
