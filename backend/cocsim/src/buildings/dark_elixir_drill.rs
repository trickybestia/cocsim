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

struct DarkElixirDrillLevel {
    pub health: f32,
}

const DARK_ELIXIR_DRILL_LEVELS_LEN: usize = 10;
const DARK_ELIXIR_DRILL_LEVEL_INDEX_MAX: usize = DARK_ELIXIR_DRILL_LEVELS_LEN - 1;
const DARK_ELIXIR_DRILL_LEVELS: [DarkElixirDrillLevel; DARK_ELIXIR_DRILL_LEVELS_LEN] = [
    DarkElixirDrillLevel { health: 800.0 },
    DarkElixirDrillLevel { health: 860.0 },
    DarkElixirDrillLevel { health: 920.0 },
    DarkElixirDrillLevel { health: 980.0 },
    DarkElixirDrillLevel { health: 1060.0 },
    DarkElixirDrillLevel { health: 1160.0 },
    DarkElixirDrillLevel { health: 1280.0 },
    DarkElixirDrillLevel { health: 1380.0 },
    DarkElixirDrillLevel { health: 1480.0 },
    DarkElixirDrillLevel { health: 1550.0 },
];

const DARK_ELIXIR_DRILL: BuildingType = BuildingType {
    name: "DarkElixirDrill",
    size: Vector2::new(3, 3),
    levels: DARK_ELIXIR_DRILL_LEVELS.len(),
    options: &[],
};

inventory::submit! {DARK_ELIXIR_DRILL}

#[derive(Serialize, Deserialize, Debug, Arbitrary, Clone)]
pub struct DarkElixirDrillModel {
    pub level: UsizeWithMax<DARK_ELIXIR_DRILL_LEVEL_INDEX_MAX>,
}

impl BuildingModel for DarkElixirDrillModel {
    fn r#type(&self) -> &'static BuildingType {
        &DARK_ELIXIR_DRILL
    }

    fn spawn(&self, world: &mut World, position: Vector2<usize>) {
        spawn_resource_building(
            world,
            DARK_ELIXIR_DRILL_LEVELS[*self.level].health,
            position,
            DARK_ELIXIR_DRILL.size,
        );
    }
}
