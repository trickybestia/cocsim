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
    game::features::{
        actions::SplashProjectileAttack,
        attack::BuildingRetargetCondition,
        targeting::building::BuildingFindTarget,
    },
};

struct WizardTowerLevel {
    pub health: f32,
    pub attack_damage: f32,
}

const WIZARD_TOWER_LEVELS_LEN: usize = 17;
const WIZARD_TOWER_LEVEL_INDEX_MAX: usize = WIZARD_TOWER_LEVELS_LEN - 1;
const WIZARD_TOWER_LEVELS: [WizardTowerLevel; WIZARD_TOWER_LEVELS_LEN] = [
    WizardTowerLevel {
        health: 620.0,
        attack_damage: 14.3,
    },
    WizardTowerLevel {
        health: 650.0,
        attack_damage: 16.9,
    },
    WizardTowerLevel {
        health: 680.0,
        attack_damage: 20.8,
    },
    WizardTowerLevel {
        health: 730.0,
        attack_damage: 26.0,
    },
    WizardTowerLevel {
        health: 840.0,
        attack_damage: 31.2,
    },
    WizardTowerLevel {
        health: 960.0,
        attack_damage: 41.6,
    },
    WizardTowerLevel {
        health: 1200.0,
        attack_damage: 52.0,
    },
    WizardTowerLevel {
        health: 1440.0,
        attack_damage: 58.5,
    },
    WizardTowerLevel {
        health: 1600.0,
        attack_damage: 65.0,
    },
    WizardTowerLevel {
        health: 1900.0,
        attack_damage: 80.6,
    },
    WizardTowerLevel {
        health: 2120.0,
        attack_damage: 91.0,
    },
    WizardTowerLevel {
        health: 2240.0,
        attack_damage: 101.4,
    },
    WizardTowerLevel {
        health: 2500.0,
        attack_damage: 109.2,
    },
    WizardTowerLevel {
        health: 2800.0,
        attack_damage: 117.0,
    },
    WizardTowerLevel {
        health: 3000.0,
        attack_damage: 123.5,
    },
    WizardTowerLevel {
        health: 3150.0,
        attack_damage: 132.6,
    },
    WizardTowerLevel {
        health: 3300.0,
        attack_damage: 143.0,
    },
];

const WIZARD_TOWER: BuildingType = BuildingType {
    name: "WizardTower",
    size: Vector2::new(3, 3),
    levels: WIZARD_TOWER_LEVELS.len(),
    options: &[],
};

inventory::submit! {WIZARD_TOWER}

const WIZARD_TOWER_MIN_ATTACK_RANGE: f32 = 0.0;
const WIZARD_TOWER_MAX_ATTACK_RANGE: f32 = 7.0;
const WIZARD_TOWER_ATTACK_COOLDOWN: f32 = 1.3;
const WIZARD_TOWER_SPLASH_ATTACK_RADIUS: f32 = 1.0;

#[derive(Serialize, Deserialize, Debug, Arbitrary, Clone)]
pub struct WizardTowerModel {
    pub x: UsizeWithMax<MAX_BUILDING_POS>,
    pub y: UsizeWithMax<MAX_BUILDING_POS>,
    pub level: UsizeWithMax<WIZARD_TOWER_LEVEL_INDEX_MAX>,
}

impl BuildingModel for WizardTowerModel {
    fn r#type(&self) -> &'static BuildingType {
        &WIZARD_TOWER
    }

    fn position(&self) -> Vector2<usize> {
        Vector2::new(*self.x, *self.y)
    }

    fn create_building(&self, world: &mut World) {
        let projectile_speed = if *self.level >= 4 { 9.0 } else { 5.0 };
        let level = &WIZARD_TOWER_LEVELS[*self.level];

        let id = create_active_building(
            world,
            level.health,
            Vector2::new(*self.x, *self.y),
            WIZARD_TOWER.size,
            WIZARD_TOWER_ATTACK_COOLDOWN,
            BuildingRetargetCondition {
                min_attack_range: WIZARD_TOWER_MIN_ATTACK_RANGE,
                max_attack_range: WIZARD_TOWER_MAX_ATTACK_RANGE,
                rotation_angle: None,
            }
            .into(),
            SplashProjectileAttack {
                damage: level.attack_damage,
                damage_radius: WIZARD_TOWER_SPLASH_ATTACK_RADIUS,
                damage_air: true,
                damage_ground: true,
                projectile_speed,
            }
            .into(),
        );

        world.add_component(
            id,
            BuildingFindTarget {
                attack_air: true,
                attack_ground: true,
                rotation_angle: None,
                min_attack_range: WIZARD_TOWER_MIN_ATTACK_RANGE,
                max_attack_range: WIZARD_TOWER_MAX_ATTACK_RANGE,
            },
        );
    }
}
