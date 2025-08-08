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
    buildings::utils::other_building::spawn_other_building,
    consts::MAX_BUILDING_POS,
};

struct TownHallLevel {
    pub health: f32,
}

const TOWN_HALL_LEVELS_LEN: usize = 17;
const TOWN_HALL_LEVEL_INDEX_MAX: usize = TOWN_HALL_LEVELS_LEN - 1;
const TOWN_HALL_LEVELS: [TownHallLevel; TOWN_HALL_LEVELS_LEN] = [
    TownHallLevel { health: 450.0 },
    TownHallLevel { health: 1600.0 },
    TownHallLevel { health: 1850.0 },
    TownHallLevel { health: 2100.0 },
    TownHallLevel { health: 2400.0 },
    TownHallLevel { health: 2800.0 },
    TownHallLevel { health: 3300.0 },
    TownHallLevel { health: 3900.0 },
    TownHallLevel { health: 4600.0 },
    TownHallLevel { health: 5500.0 },
    TownHallLevel { health: 6800.0 },
    TownHallLevel { health: 7500.0 },
    TownHallLevel { health: 8200.0 },
    TownHallLevel { health: 8900.0 },
    TownHallLevel { health: 9600.0 },
    TownHallLevel { health: 10000.0 },
    TownHallLevel { health: 10400.0 },
];

const TOWN_HALL: BuildingType = BuildingType {
    name: "TownHall",
    size: Vector2::new(4, 4),
    levels: TOWN_HALL_LEVELS.len(),
    options: &[],
};

inventory::submit! {TOWN_HALL}

#[derive(Serialize, Deserialize, Debug, Arbitrary, Clone)]
pub struct TownHallModel {
    pub x: UsizeWithMax<MAX_BUILDING_POS>,
    pub y: UsizeWithMax<MAX_BUILDING_POS>,
    pub level: UsizeWithMax<TOWN_HALL_LEVEL_INDEX_MAX>,
}

impl BuildingModel for TownHallModel {
    fn r#type(&self) -> &'static BuildingType {
        &TOWN_HALL
    }

    fn position(&self) -> Vector2<usize> {
        Vector2::new(*self.x, *self.y)
    }

    fn spawn(&self, world: &mut World) {
        let id = spawn_other_building(
            world,
            TOWN_HALL_LEVELS[*self.level].health,
            Vector2::new(*self.x, *self.y),
            TOWN_HALL.size,
        );

        world
            .insert_one(id, crate::game::features::buildings::TownHall)
            .unwrap();
    }
}
