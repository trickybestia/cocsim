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
    UsizeWithMax,
    buildings::utils::active_building::create_active_building,
    consts::MAX_BUILDING_POS,
    game::features::actions::{
        BuildingFindTarget,
        SplashProjectileAttack,
    },
};

struct MortarLevel {
    pub health: f32,
    pub attack_damage: f32,
}

const MORTAR_LEVELS_LEN: usize = 17;
const MORTAR_LEVEL_INDEX_MAX: usize = MORTAR_LEVELS_LEN - 1;
const MORTAR_LEVELS: [MortarLevel; MORTAR_LEVELS_LEN] = [
    MortarLevel {
        health: 400.0,
        attack_damage: 20.0,
    },
    MortarLevel {
        health: 450.0,
        attack_damage: 25.0,
    },
    MortarLevel {
        health: 500.0,
        attack_damage: 30.0,
    },
    MortarLevel {
        health: 550.0,
        attack_damage: 35.0,
    },
    MortarLevel {
        health: 600.0,
        attack_damage: 45.0,
    },
    MortarLevel {
        health: 650.0,
        attack_damage: 55.0,
    },
    MortarLevel {
        health: 700.0,
        attack_damage: 75.0,
    },
    MortarLevel {
        health: 800.0,
        attack_damage: 100.0,
    },
    MortarLevel {
        health: 950.0,
        attack_damage: 125.0,
    },
    MortarLevel {
        health: 1100.0,
        attack_damage: 150.0,
    },
    MortarLevel {
        health: 1300.0,
        attack_damage: 175.0,
    },
    MortarLevel {
        health: 1500.0,
        attack_damage: 190.0,
    },
    MortarLevel {
        health: 1700.0,
        attack_damage: 210.0,
    },
    MortarLevel {
        health: 1950.0,
        attack_damage: 240.0,
    },
    MortarLevel {
        health: 2150.0,
        attack_damage: 270.0,
    },
    MortarLevel {
        health: 2300.0,
        attack_damage: 300.0,
    },
    MortarLevel {
        health: 2450.0,
        attack_damage: 330.0,
    },
];

const MORTAR: BuildingType = BuildingType {
    name: "Mortar",
    size: Vector2::new(3, 3),
    levels: MORTAR_LEVELS.len(),
    options: &[],
};

inventory::submit! {MORTAR}

const MORTAR_MIN_ATTACK_RANGE: f32 = 4.0;
const MORTAR_MAX_ATTACK_RANGE: f32 = 11.0;
const MORTAR_ATTACK_COOLDOWN: f32 = 5.0;
const MORTAR_PROJECTILE_SPEED: f32 = 5.0;
const MORTAR_SPLASH_ATTACK_RADIUS: f32 = 1.5;

#[derive(Serialize, Deserialize, Debug, Arbitrary)]
pub struct MortarModel {
    pub x: UsizeWithMax<MAX_BUILDING_POS>,
    pub y: UsizeWithMax<MAX_BUILDING_POS>,
    pub level: UsizeWithMax<MORTAR_LEVEL_INDEX_MAX>,
}

impl BuildingModel for MortarModel {
    fn r#type(&self) -> &'static BuildingType {
        &MORTAR
    }

    fn position(&self) -> Vector2<usize> {
        Vector2::new(*self.x, *self.y)
    }

    fn create_building(&self, world: &mut World) {
        let level = &MORTAR_LEVELS[*self.level];

        create_active_building(
            world,
            level.health,
            Vector2::new(*self.x, *self.y),
            MORTAR.size,
            MORTAR_MIN_ATTACK_RANGE,
            MORTAR_MAX_ATTACK_RANGE,
            MORTAR_ATTACK_COOLDOWN,
            BuildingFindTarget {
                attack_air: false,
                attack_ground: true,
            }
            .into(),
            SplashProjectileAttack {
                damage: level.attack_damage,
                damage_radius: MORTAR_SPLASH_ATTACK_RADIUS,
                damage_air: false,
                damage_ground: true,
                projectile_speed: MORTAR_PROJECTILE_SPEED,
            }
            .into(),
        );
    }
}
