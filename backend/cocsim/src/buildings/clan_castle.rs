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

struct ClanCastleLevel {
    pub health: f32,
}

const CLAN_CASTLE_LEVELS: &[ClanCastleLevel] = &[
    ClanCastleLevel { health: 1000.0 },
    ClanCastleLevel { health: 1400.0 },
    ClanCastleLevel { health: 2000.0 },
    ClanCastleLevel { health: 2600.0 },
    ClanCastleLevel { health: 3000.0 },
    ClanCastleLevel { health: 3400.0 },
    ClanCastleLevel { health: 4000.0 },
    ClanCastleLevel { health: 4400.0 },
    ClanCastleLevel { health: 4800.0 },
    ClanCastleLevel { health: 5200.0 },
    ClanCastleLevel { health: 5400.0 },
    ClanCastleLevel { health: 5600.0 },
    ClanCastleLevel { health: 5800.0 },
];

const CLAN_CASTLE: BuildingType = BuildingType {
    name: "ClanCastle",
    size: Vector2::new(3, 3),
    levels: CLAN_CASTLE_LEVELS.len(),
    options: &[],
};

inventory::submit! {CLAN_CASTLE}

#[derive(Serialize, Deserialize, Debug, Arbitrary)]
pub struct ClanCastleModel {
    pub x: usize,
    pub y: usize,
    pub level: usize,
}

impl BuildingModel for ClanCastleModel {
    fn create_building(&self, world: &mut World) -> Result<()> {
        create_passive_building(
            world,
            CLAN_CASTLE_LEVELS
                .get(self.level)
                .context("Level out of range")?
                .health,
            Vector2::new(self.x, self.y),
            CLAN_CASTLE.size,
            None,
        )?;

        Ok(())
    }
}
