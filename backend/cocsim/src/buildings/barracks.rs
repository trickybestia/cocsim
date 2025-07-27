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

struct BarracksLevel {
    pub health: f32,
}

const BARRACKS_LEVELS: &[BarracksLevel] = &[
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
};

inventory::submit! {BARRACKS}

#[derive(Serialize, Deserialize, Debug, Arbitrary)]
pub struct BarracksModel {
    pub x: usize,
    pub y: usize,
    pub level: usize,
}

impl BuildingModel for BarracksModel {
    fn create_building(&self, world: &mut World) -> Result<()> {
        create_passive_building(
            world,
            BARRACKS_LEVELS
                .get(self.level)
                .context("Level out of range")?
                .health,
            Vector2::new(self.x, self.y),
            BARRACKS.size,
            None,
        )?;

        Ok(())
    }
}
