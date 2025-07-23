use anyhow::{
    Context,
    Result,
};
use nalgebra::Vector2;
use serde::{
    Deserialize,
    Serialize,
};
use shipyard::World;

use crate::{
    BuildingModel,
    BuildingType,
    buildings::utils::passive_building::create_passive_building,
};

struct TownHallLevel {
    pub health: f32,
}

const TOWN_HALL_LEVELS: &[TownHallLevel] = &[
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

#[derive(Serialize, Deserialize, Debug)]
pub struct TownHallModel {
    pub x: usize,
    pub y: usize,
    pub level: usize,
}

impl BuildingModel for TownHallModel {
    fn create_building(&self, world: &mut World) -> Result<()> {
        let id = create_passive_building(
            world,
            TOWN_HALL_LEVELS
                .get(self.level)
                .context("Level out of range")?
                .health,
            Vector2::new(self.x, self.y),
            TOWN_HALL.size,
            None,
        )?;

        world.add_component(id, crate::game::features::buildings::TownHall);

        Ok(())
    }
}
