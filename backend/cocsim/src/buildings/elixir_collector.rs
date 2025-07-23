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

struct ElixirCollectorLevel {
    pub health: f32,
}

const ELIXIR_COLLECTOR_LEVELS: &[ElixirCollectorLevel] = &[
    ElixirCollectorLevel { health: 400.0 },
    ElixirCollectorLevel { health: 440.0 },
    ElixirCollectorLevel { health: 480.0 },
    ElixirCollectorLevel { health: 520.0 },
    ElixirCollectorLevel { health: 560.0 },
    ElixirCollectorLevel { health: 600.0 },
    ElixirCollectorLevel { health: 640.0 },
    ElixirCollectorLevel { health: 680.0 },
    ElixirCollectorLevel { health: 720.0 },
    ElixirCollectorLevel { health: 780.0 },
    ElixirCollectorLevel { health: 860.0 },
    ElixirCollectorLevel { health: 960.0 },
    ElixirCollectorLevel { health: 1080.0 },
    ElixirCollectorLevel { health: 1180.0 },
    ElixirCollectorLevel { health: 1280.0 },
    ElixirCollectorLevel { health: 1350.0 },
];

const ELIXIR_COLLECTOR: BuildingType = BuildingType {
    name: "ElixirCollector",
    size: Vector2::new(3, 3),
    levels: ELIXIR_COLLECTOR_LEVELS.len(),
    options: &[],
};

inventory::submit! {ELIXIR_COLLECTOR}

#[derive(Serialize, Deserialize, Debug)]
pub struct ElixirCollectorModel {
    pub x: usize,
    pub y: usize,
    pub level: usize,
}

impl BuildingModel for ElixirCollectorModel {
    fn create_building(&self, world: &mut World) -> Result<()> {
        create_passive_building(
            world,
            ELIXIR_COLLECTOR_LEVELS
                .get(self.level)
                .context("Level out of range")?
                .health,
            Vector2::new(self.x, self.y),
            ELIXIR_COLLECTOR.size,
            None,
        )?;

        Ok(())
    }
}
