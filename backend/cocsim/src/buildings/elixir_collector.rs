use arbitrary::Arbitrary;
use nalgebra::Vector2;
use serde::{
    Deserialize,
    Serialize,
};
use hecs::World;

use crate::{
    BuildingModel,
    BuildingType,
    UsizeWithMax,
    buildings::utils::passive_building::create_passive_building,
    consts::MAX_BUILDING_POS,
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
    pub x: UsizeWithMax<MAX_BUILDING_POS>,
    pub y: UsizeWithMax<MAX_BUILDING_POS>,
    pub level: UsizeWithMax<ELIXIR_COLLECTOR_LEVEL_INDEX_MAX>,
}

impl BuildingModel for ElixirCollectorModel {
    fn r#type(&self) -> &'static BuildingType {
        &ELIXIR_COLLECTOR
    }

    fn position(&self) -> Vector2<usize> {
        Vector2::new(*self.x, *self.y)
    }

    fn spawn(&self, world: &mut World) {
        create_passive_building(
            world,
            ELIXIR_COLLECTOR_LEVELS[*self.level].health,
            Vector2::new(*self.x, *self.y),
            ELIXIR_COLLECTOR.size,
        );
    }
}
