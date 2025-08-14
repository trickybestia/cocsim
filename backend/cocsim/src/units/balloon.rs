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
    Shape,
    ShapeColor,
    UnitModel,
    UnitType,
    game::features::{
        actions::SplashDamage,
        attack::{
            Team,
            targeting::air_unit::{
                AirUnitFindTarget,
                DefensiveBuildingTargetPrioritizer,
            },
        },
        position::Position,
        to_be_despawned::OnDespawn,
    },
    units::utils::air_unit::create_air_unit,
    usize_with_max::UsizeWithMax,
};

struct BalloonLevel {
    pub health: f32,
    pub attack_damage: f32,
    pub death_damage: f32,
}

const BALLON_LEVELS_LEN: usize = 11;
const BALLOON_LEVEL_INDEX_MAX: usize = BALLON_LEVELS_LEN - 1;
const BALLOON_LEVELS: [BalloonLevel; BALLON_LEVELS_LEN] = [
    BalloonLevel {
        health: 150.0,
        attack_damage: 75.0,
        death_damage: 25.0,
    },
    BalloonLevel {
        health: 180.0,
        attack_damage: 96.0,
        death_damage: 32.0,
    },
    BalloonLevel {
        health: 216.0,
        attack_damage: 144.0,
        death_damage: 48.0,
    },
    BalloonLevel {
        health: 280.0,
        attack_damage: 216.0,
        death_damage: 72.0,
    },
    BalloonLevel {
        health: 390.0,
        attack_damage: 324.0,
        death_damage: 108.0,
    },
    BalloonLevel {
        health: 545.0,
        attack_damage: 486.0,
        death_damage: 162.0,
    },
    BalloonLevel {
        health: 690.0,
        attack_damage: 594.0,
        death_damage: 214.0,
    },
    BalloonLevel {
        health: 840.0,
        attack_damage: 708.0,
        death_damage: 268.0,
    },
    BalloonLevel {
        health: 940.0,
        attack_damage: 768.0,
        death_damage: 322.0,
    },
    BalloonLevel {
        health: 1040.0,
        attack_damage: 828.0,
        death_damage: 352.0,
    },
    BalloonLevel {
        health: 1140.0,
        attack_damage: 870.0,
        death_damage: 375.0,
    },
];

const BALLOON: UnitType = UnitType {
    name: "Balloon",
    housing_space: 5,
    levels: BALLOON_LEVELS.len(),
    clan_castle_deployment_priority: 1,
    attack_air: false,
    attack_ground: true,
};

inventory::submit! {BALLOON}

const BALLOON_SPEED: f32 = 1.3;
const BALLOON_ATTACK_COOLDOWN: f32 = 3.0;
const BALLOON_ATTACK_RANGE: f32 = 0.0;
const BALLOON_SPLASH_ATTACK_RADIUS: f32 = 1.2;

fn draw_balloon(id: Entity, world: &World, result: &mut Vec<Shape>) {
    let position = world.get::<&Position>(id).unwrap().0;

    result.push(Shape::Circle {
        x: position.x,
        y: position.y,
        radius: 0.25,
        color: ShapeColor::new(0, 0, 0),
    });
}

#[derive(Serialize, Deserialize, Debug, Clone, Arbitrary)]
pub struct BalloonModel {
    pub level: UsizeWithMax<BALLOON_LEVEL_INDEX_MAX>,
}

impl UnitModel for BalloonModel {
    fn r#type(&self) -> &'static UnitType {
        &BALLOON
    }

    fn level(&self) -> usize {
        *self.level
    }

    fn spawn(&self, world: &mut World, position: Vector2<f32>, team: Team) {
        let level = &BALLOON_LEVELS[*self.level];

        let id = create_air_unit(
            world,
            position,
            level.health,
            BALLOON_SPEED,
            BALLOON_ATTACK_COOLDOWN,
            Box::new(SplashDamage {
                damage_ground: true,
                damage_air: false,
                damage: level.attack_damage,
                radius: BALLOON_SPLASH_ATTACK_RADIUS,
            }),
            draw_balloon,
            team,
            BALLOON.housing_space,
        );

        world
            .insert(
                id,
                (
                    AirUnitFindTarget {
                        prioritizer: DefensiveBuildingTargetPrioritizer.into(),
                        attack_range: BALLOON_ATTACK_RANGE,
                    },
                    OnDespawn(Box::new(SplashDamage {
                        damage_ground: true,
                        damage_air: false,
                        damage: level.death_damage,
                        radius: BALLOON_SPLASH_ATTACK_RADIUS,
                    })),
                ),
            )
            .unwrap();
    }
}
