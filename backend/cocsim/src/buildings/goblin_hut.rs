use anyhow::ensure;
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

struct GoblinHutLevel {
    pub health: f32,
}

const GOBLIN_HUT_LEVELS: &[GoblinHutLevel] = &[GoblinHutLevel { health: 250.0 }];

const GOBLIN_HUT: BuildingType = BuildingType {
    name: "GoblinHut",
    size: Vector2::new(2, 2),
    levels: GOBLIN_HUT_LEVELS.len(),
    options: &[],
};

inventory::submit! {GOBLIN_HUT}

#[derive(Serialize, Deserialize, Debug, Arbitrary)]
pub struct GoblinHutModel {
    pub x: usize,
    pub y: usize,
    pub level: usize,
}

impl BuildingModel for GoblinHutModel {
    fn r#type(&self) -> &'static BuildingType {
        &GOBLIN_HUT
    }

    fn position(&self) -> Vector2<usize> {
        Vector2::new(self.x, self.y)
    }

    fn validate(&self) -> anyhow::Result<()> {
        ensure!(self.level < GOBLIN_HUT_LEVELS.len());

        Ok(())
    }

    fn create_building(&self, world: &mut World) {
        create_passive_building(
            world,
            GOBLIN_HUT_LEVELS[self.level].health,
            Vector2::new(self.x, self.y),
            GOBLIN_HUT.size,
            None,
        );
    }
}
