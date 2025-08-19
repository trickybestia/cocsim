use arbitrary::Arbitrary;
use hecs::{
    Entity,
    World,
};
use nalgebra::Vector2;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    BuildingModel,
    BuildingType,
    Game,
    ShapeColor,
    UsizeWithMax,
    buildings::utils::{
        defensive_building::spawn_defensive_building,
        trap::spawn_trap,
    },
    game::features::{
        actions::{
            Action,
            WithDespawn,
        },
        attack::{
            Attacker,
            BuildingRetargetCondition,
            force_retarget,
            targeting::building::BuildingFindTarget,
        },
        buildings::Building,
        delay::Delay,
        drawable::Line,
        health::Health,
    },
};

struct HiddenTeslaLevel {
    pub health: f32,
    pub attack_damage: f32,
}

const HIDDEN_TESLA_LEVELS_LEN: usize = 16;
const HIDDEN_TESLA_LEVEL_INDEX_MAX: usize = HIDDEN_TESLA_LEVELS_LEN - 1;
const HIDDEN_TESLA_LEVELS: [HiddenTeslaLevel; HIDDEN_TESLA_LEVELS_LEN] = [
    HiddenTeslaLevel {
        health: 600.0,
        attack_damage: 20.4,
    },
    HiddenTeslaLevel {
        health: 630.0,
        attack_damage: 24.0,
    },
    HiddenTeslaLevel {
        health: 660.0,
        attack_damage: 28.8,
    },
    HiddenTeslaLevel {
        health: 690.0,
        attack_damage: 33.0,
    },
    HiddenTeslaLevel {
        health: 730.0,
        attack_damage: 38.4,
    },
    HiddenTeslaLevel {
        health: 770.0,
        attack_damage: 45.0,
    },
    HiddenTeslaLevel {
        health: 810.0,
        attack_damage: 52.2,
    },
    HiddenTeslaLevel {
        health: 850.0,
        attack_damage: 59.4,
    },
    HiddenTeslaLevel {
        health: 900.0,
        attack_damage: 66.0,
    },
    HiddenTeslaLevel {
        health: 980.0,
        attack_damage: 72.0,
    },
    HiddenTeslaLevel {
        health: 1100.0,
        attack_damage: 78.0,
    },
    HiddenTeslaLevel {
        health: 1200.0,
        attack_damage: 84.0,
    },
    HiddenTeslaLevel {
        health: 1350.0,
        attack_damage: 90.0,
    },
    HiddenTeslaLevel {
        health: 1450.0,
        attack_damage: 96.0,
    },
    HiddenTeslaLevel {
        health: 1550.0,
        attack_damage: 102.0,
    },
    HiddenTeslaLevel {
        health: 1650.0,
        attack_damage: 108.0,
    },
];

const HIDDEN_TESLA: BuildingType = BuildingType {
    name: "HiddenTesla",
    size: Vector2::new(2, 2),
    levels: HIDDEN_TESLA_LEVELS.len(),
    options: &[],
};

inventory::submit! {HIDDEN_TESLA}

const HIDDEN_TESLA_MIN_ATTACK_RANGE: f32 = 0.0;
const HIDDEN_TESLA_MAX_ATTACK_RANGE: f32 = 7.0;
const HIDDEN_TESLA_ATTACK_COOLDOWN: f32 = 0.6;
const HIDDEN_TESLA_TRIGGER_RADIUS: f32 = 6.0;

#[derive(Serialize, Deserialize, Debug, Arbitrary, Clone)]
pub struct HiddenTeslaModel {
    pub level: UsizeWithMax<HIDDEN_TESLA_LEVEL_INDEX_MAX>,
}

#[derive(Debug, Clone)]
struct HiddenTeslaAttack {
    pub damage: f32,
}

impl Action for HiddenTeslaAttack {
    fn call(&self, actor: Entity, game: &mut Game) {
        let target = game.world.get::<&Attacker>(actor).unwrap().target;
        let mut target_health = game.world.get::<&mut Health>(target).unwrap();

        target_health.incoming_damage += self.damage;

        drop(target_health);

        game.world.spawn((
            Delay { time_left: 0.25 },
            Line {
                a: actor,
                b: target,
                width: 0.1,
                color: ShapeColor::new(0, 255, 255),
            },
        ));
    }
}

#[derive(Debug, Clone)]
struct SpawnHiddenTesla {
    pub level: UsizeWithMax<HIDDEN_TESLA_LEVEL_INDEX_MAX>,
}

impl Action for SpawnHiddenTesla {
    fn call(&self, actor: Entity, game: &mut Game) {
        let level = &HIDDEN_TESLA_LEVELS[*self.level];

        let position = game.world.get::<&Building>(actor).unwrap().position;

        let id = spawn_defensive_building(
            &mut game.world,
            level.health,
            position,
            HIDDEN_TESLA.size,
            HIDDEN_TESLA_ATTACK_COOLDOWN,
            BuildingRetargetCondition {
                min_attack_range: HIDDEN_TESLA_MIN_ATTACK_RANGE,
                max_attack_range: HIDDEN_TESLA_MAX_ATTACK_RANGE,
                rotation_angle: None,
            }
            .into(),
            Box::new(HiddenTeslaAttack {
                damage: level.attack_damage,
            }),
        );

        game.world
            .insert_one(
                id,
                BuildingFindTarget {
                    attack_air: true,
                    attack_ground: true,
                    rotation_angle: None,
                    min_attack_range: HIDDEN_TESLA_MIN_ATTACK_RANGE,
                    max_attack_range: HIDDEN_TESLA_MAX_ATTACK_RANGE,
                    min_housing_space: 0,
                },
            )
            .unwrap();

        game.update_collision_grid();

        force_retarget(game);
    }
}

impl BuildingModel for HiddenTeslaModel {
    fn r#type(&self) -> &'static BuildingType {
        &HIDDEN_TESLA
    }

    fn spawn(&self, world: &mut World, position: Vector2<usize>) {
        let id = spawn_trap(
            world,
            position,
            HIDDEN_TESLA.size,
            Box::new(WithDespawn(Box::new(SpawnHiddenTesla {
                level: self.level,
            }))),
        );

        world
            .insert_one(
                id,
                BuildingFindTarget {
                    attack_air: true,
                    attack_ground: true,
                    rotation_angle: None,
                    min_attack_range: 0.0,
                    max_attack_range: HIDDEN_TESLA_TRIGGER_RADIUS,
                    min_housing_space: 0,
                },
            )
            .unwrap();

        world
            .get::<&mut Building>(id)
            .unwrap()
            .affects_percentage_destroyed = true;
    }
}
