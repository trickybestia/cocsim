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
    consts::MAX_BUILDING_POS,
};

struct ClanCastleLevel {
    pub health: f32,
}

const CLAN_CASTLE_LEVELS_LEN: usize = 13;
const CLAN_CASTLE_LEVEL_INDEX_MAX: usize = CLAN_CASTLE_LEVELS_LEN - 1;
const CLAN_CASTLE_LEVELS: [ClanCastleLevel; CLAN_CASTLE_LEVELS_LEN] = [
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

#[derive(Serialize, Deserialize, Debug, Arbitrary, Clone)]
pub struct ClanCastleModel {
    pub x: UsizeWithMax<MAX_BUILDING_POS>,
    pub y: UsizeWithMax<MAX_BUILDING_POS>,
    pub level: UsizeWithMax<CLAN_CASTLE_LEVEL_INDEX_MAX>,
}

impl BuildingModel for ClanCastleModel {
    fn r#type(&self) -> &'static BuildingType {
        &CLAN_CASTLE
    }

    fn position(&self) -> Vector2<usize> {
        Vector2::new(*self.x, *self.y)
    }

    fn spawn(&self, world: &mut World) {
        spawn_resource_building(
            world,
            CLAN_CASTLE_LEVELS[*self.level].health,
            Vector2::new(*self.x, *self.y),
            CLAN_CASTLE.size,
        );
    }
}
