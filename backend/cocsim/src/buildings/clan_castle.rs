use std::{
    iter::repeat,
    ops::Deref,
};

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
    UnitModelEnum,
    UsizeWithMax,
    WithCount,
    WithMaxHousingSpace,
    buildings::utils::resource_building::spawn_resource_building,
    consts::MAX_CLAN_CASTLE_HOUSING_SPACE,
    game::features::clan_castle::ClanCastle,
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
    affects_drop_zone: true,
};

inventory::submit! {CLAN_CASTLE}

const CLAN_CASTLE_UNIT_DEPLOY_TRIGGER_RANGE: f32 = 12.0;
const CLAN_CASTLE_UNIT_DEPLOY_COOLDOWN: f32 = 0.25;

#[derive(Serialize, Deserialize, Debug, Arbitrary, Clone)]
pub struct ClanCastleModel {
    pub level: UsizeWithMax<CLAN_CASTLE_LEVEL_INDEX_MAX>,
    pub units: WithMaxHousingSpace<MAX_CLAN_CASTLE_HOUSING_SPACE, WithCount<UnitModelEnum>>,
}

impl BuildingModel for ClanCastleModel {
    fn r#type(&self) -> &'static BuildingType {
        &CLAN_CASTLE
    }

    fn spawn(&self, world: &mut World, position: Vector2<usize>) {
        let id = spawn_resource_building(
            world,
            CLAN_CASTLE_LEVELS[*self.level].health,
            position,
            CLAN_CASTLE.size,
        );

        let mut units = self
            .units
            .deref()
            .iter()
            .map(|unit_with_count| {
                repeat(unit_with_count.value.clone()).take(unit_with_count.count)
            })
            .flatten()
            .collect::<Vec<_>>();
        units.sort_unstable_by(|a, b| b.clan_castle_deployment_cmp(a));

        world
            .insert_one(
                id,
                ClanCastle {
                    units,
                    unit_deploy_cooldown: CLAN_CASTLE_UNIT_DEPLOY_COOLDOWN,
                    remaining_unit_deploy_cooldown: 0.0,
                    unit_deploy_trigger_range: CLAN_CASTLE_UNIT_DEPLOY_TRIGGER_RANGE,
                },
            )
            .unwrap();
    }
}
