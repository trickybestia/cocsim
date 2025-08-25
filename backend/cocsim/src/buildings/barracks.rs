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

struct BarracksLevel {
    pub health: f32,
}

const BARRACKS_LEVELS_LEN: usize = 18;
const BARRACKS_LEVEL_INDEX_MAX: usize = BARRACKS_LEVELS_LEN - 1;
const BARRACKS_LEVELS: [BarracksLevel; BARRACKS_LEVELS_LEN] = [
    BarracksLevel { health: 250.0 },
    BarracksLevel { health: 290.0 },
    BarracksLevel { health: 330.0 },
    BarracksLevel { health: 370.0 },
    BarracksLevel { health: 420.0 },
    BarracksLevel { health: 470.0 },
    BarracksLevel { health: 520.0 },
    BarracksLevel { health: 580.0 },
    BarracksLevel { health: 650.0 },
    BarracksLevel { health: 730.0 },
    BarracksLevel { health: 810.0 },
    BarracksLevel { health: 900.0 },
    BarracksLevel { health: 980.0 },
    BarracksLevel { health: 1050.0 },
    BarracksLevel { health: 1150.0 },
    BarracksLevel { health: 1250.0 },
    BarracksLevel { health: 1350.0 },
    BarracksLevel { health: 1450.0 },
];

const BARRACKS: BuildingType = BuildingType {
    name: "Barracks",
    size: Vector2::new(3, 3),
    levels: BARRACKS_LEVELS.len(),
    options: &[],
    affects_drop_zone: true,
};

inventory::submit! {BARRACKS}

#[derive(Serialize, Deserialize, Debug, Arbitrary, Clone)]
pub struct BarracksModel {
    pub level: UsizeWithMax<BARRACKS_LEVEL_INDEX_MAX>,
}

impl BuildingModel for BarracksModel {
    fn r#type(&self) -> &'static BuildingType {
        &BARRACKS
    }

    fn spawn(&self, world: &mut World, position: Vector2<usize>) {
        spawn_other_building(
            world,
            BARRACKS_LEVELS[*self.level].health,
            position,
            BARRACKS.size,
        );
    }
}
