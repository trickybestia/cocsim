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

struct WallLevel {
    pub health: f32,
}

const WALL_LEVELS: &[WallLevel] = &[
    WallLevel { health: 300.0 },
    WallLevel { health: 500.0 },
    WallLevel { health: 700.0 },
    WallLevel { health: 900.0 },
    WallLevel { health: 1400.0 },
    WallLevel { health: 2000.0 },
    WallLevel { health: 2500.0 },
    WallLevel { health: 3000.0 },
    WallLevel { health: 3500.0 },
    WallLevel { health: 4000.0 },
    WallLevel { health: 5000.0 },
    WallLevel { health: 7000.0 },
    WallLevel { health: 9000.0 },
    WallLevel { health: 11000.0 },
    WallLevel { health: 12500.0 },
    WallLevel { health: 13500.0 },
    WallLevel { health: 14500.0 },
    WallLevel { health: 15500.0 },
];

const WALL: &BuildingType = &BuildingType {
    name: "Wall",
    size: Vector2::new(1, 1),
    levels: WALL_LEVELS.len(),
    options: &[],
};

#[derive(Serialize, Deserialize, Debug)]
pub struct WallModel {
    pub x: usize,
    pub y: usize,
    pub level: usize,
}

impl BuildingModel for WallModel {
    fn create_building(&self, world: &mut World) -> Result<()> {
        create_passive_building(
            world,
            WALL_LEVELS
                .get(self.level)
                .context("Level out of range")?
                .health,
            Vector2::new(self.x, self.y),
            WALL.size,
            None,
        )?;

        Ok(())
    }
}
