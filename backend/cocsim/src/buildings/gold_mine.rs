use anyhow::{
    Context,
    Result,
};
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
    buildings::utils::passive_building::create_passive_building,
};

struct GoldMineLevel {
    pub health: f32,
}

const GOLD_MINE_LEVELS: &[GoldMineLevel] = &[
    GoldMineLevel { health: 400.0 },
    GoldMineLevel { health: 440.0 },
    GoldMineLevel { health: 480.0 },
    GoldMineLevel { health: 520.0 },
    GoldMineLevel { health: 560.0 },
    GoldMineLevel { health: 600.0 },
    GoldMineLevel { health: 640.0 },
    GoldMineLevel { health: 680.0 },
    GoldMineLevel { health: 720.0 },
    GoldMineLevel { health: 780.0 },
    GoldMineLevel { health: 860.0 },
    GoldMineLevel { health: 960.0 },
    GoldMineLevel { health: 1080.0 },
    GoldMineLevel { health: 1180.0 },
    GoldMineLevel { health: 1280.0 },
    GoldMineLevel { health: 1350.0 },
];

const GOLD_MINE: BuildingType = BuildingType {
    name: "GoldMine",
    size: Vector2::new(3, 3),
    levels: GOLD_MINE_LEVELS.len(),
    options: &[],
};

inventory::submit! {GOLD_MINE}

#[derive(Serialize, Deserialize, Debug, Arbitrary)]
pub struct GoldMineModel {
    pub x: usize,
    pub y: usize,
    pub level: usize,
}

impl BuildingModel for GoldMineModel {
    fn create_building(&self, world: &mut World) -> Result<()> {
        create_passive_building(
            world,
            GOLD_MINE_LEVELS
                .get(self.level)
                .context("Level out of range")?
                .health,
            Vector2::new(self.x, self.y),
            GOLD_MINE.size,
            None,
        )?;

        Ok(())
    }
}
