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
    affects_drop_zone: true,
};

inventory::submit! {ARMY_CAMP}

#[derive(Serialize, Deserialize, Debug, Arbitrary, Clone)]
pub struct ArmyCampModel {
    pub level: UsizeWithMax<ARMY_CAMP_LEVEL_INDEX_MAX>,
}

impl BuildingModel for ArmyCampModel {
    fn r#type(&self) -> &'static BuildingType {
        &ARMY_CAMP
    }

    fn spawn(&self, world: &mut World, position: Vector2<usize>) {
        spawn_other_building(
            world,
            ARMY_CAMP_LEVELS[*self.level].health,
            position,
            ARMY_CAMP.size,
        );
    }
}
