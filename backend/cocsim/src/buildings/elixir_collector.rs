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
    buildings::utils::resource_building::spawn_resource_building,
};

struct ElixirCollectorLevel {
    pub health: f32,
}

const ELIXIR_COLLECTOR_LEVELS_LEN: usize = 16;
const ELIXIR_COLLECTOR_LEVEL_INDEX_MAX: usize = ELIXIR_COLLECTOR_LEVELS_LEN - 1;
const ELIXIR_COLLECTOR_LEVELS: [ElixirCollectorLevel; ELIXIR_COLLECTOR_LEVELS_LEN] = [
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

#[derive(Serialize, Deserialize, Debug, Arbitrary, Clone)]
pub struct ElixirCollectorModel {
    pub level: UsizeWithMax<ELIXIR_COLLECTOR_LEVEL_INDEX_MAX>,
}

impl BuildingModel for ElixirCollectorModel {
    fn r#type(&self) -> &'static BuildingType {
        &ELIXIR_COLLECTOR
    }

    fn spawn(&self, world: &mut World, position: Vector2<usize>) {
        spawn_resource_building(
            world,
            ELIXIR_COLLECTOR_LEVELS[*self.level].health,
            position,
            ELIXIR_COLLECTOR.size,
        );
    }
}
