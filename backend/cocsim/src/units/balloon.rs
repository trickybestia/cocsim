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
    Game,
    Shape,
    ShapeColor,
    UnitModel,
    UnitType,
    game::features::{
        actions::SplashDamage,
        attack::targeting::air_unit::{
            ActiveBuildingTargetPrioritizer,
            AirUnitFindTarget,
        },
        position::Position,
        to_be_deleted::OnDelete,
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
};

inventory::submit! {BALLOON}

const BALLOON_SPEED: f32 = 1.3;
const BALLOON_ATTACK_COOLDOWN: f32 = 3.0;
const BALLOON_ATTACK_RANGE: f32 = 0.0;
const BALLOON_SPLASH_ATTACK_RADIUS: f32 = 1.2;

fn draw_balloon(id: Entity, game: &Game, result: &mut Vec<Shape>) {
    let position = game.world.get::<&Position>(id).unwrap().0;

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
    fn create_unit(&self, world: &mut World, position: Vector2<f32>) {
        let level = &BALLOON_LEVELS[*self.level];

        let id = create_air_unit(
            world,
            position,
            level.health,
            BALLOON_SPEED,
            BALLOON_ATTACK_COOLDOWN,
            SplashDamage {
                damage_ground: true,
                damage_air: false,
                damage: level.attack_damage,
                radius: BALLOON_SPLASH_ATTACK_RADIUS,
            }
            .into(),
            draw_balloon,
        );

        world
            .insert(
                id,
                (
                    AirUnitFindTarget {
                        prioritizer: ActiveBuildingTargetPrioritizer.into(),
                        attack_range: BALLOON_ATTACK_RANGE,
                    },
                    OnDelete(
                        SplashDamage {
                            damage_ground: true,
                            damage_air: false,
                            damage: level.death_damage,
                            radius: BALLOON_SPLASH_ATTACK_RADIUS,
                        }
                        .into(),
                    ),
                ),
            )
            .unwrap();
    }
}
