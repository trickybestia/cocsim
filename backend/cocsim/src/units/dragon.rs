use anyhow::{
    Context,
    Result,
};
use nalgebra::Vector2;
use serde::{
    Deserialize,
    Serialize,
};
use shipyard::{
    AllStoragesView,
    EntityId,
    View,
    World,
};

use crate::{
    Shape,
    ShapeColor,
    UnitModel,
    UnitType,
    game::features::{
        attack::{
            AirUnitFindTargetBehaviour,
            AttackTargetFlags,
            MeleeAttackBehaviour,
        },
        position::Position,
    },
    units::utils::air_unit::create_air_unit,
};

struct DragonLevel {
    pub health: f32,
    pub attack_damage: f32,
}

const DRAGON_LEVELS: &[DragonLevel] = &[
    DragonLevel {
        health: 1900.0,
        attack_damage: 175.0,
    },
    DragonLevel {
        health: 2100.0,
        attack_damage: 200.0,
    },
    DragonLevel {
        health: 2300.0,
        attack_damage: 225.0,
    },
    DragonLevel {
        health: 2700.0,
        attack_damage: 262.5,
    },
    DragonLevel {
        health: 3100.0,
        attack_damage: 300.0,
    },
    DragonLevel {
        health: 3400.0,
        attack_damage: 337.5,
    },
    DragonLevel {
        health: 3900.0,
        attack_damage: 387.5,
    },
    DragonLevel {
        health: 4200.0,
        attack_damage: 412.5,
    },
    DragonLevel {
        health: 4500.0,
        attack_damage: 437.5,
    },
    DragonLevel {
        health: 4900.0,
        attack_damage: 462.5,
    },
    DragonLevel {
        health: 5300.0,
        attack_damage: 487.5,
    },
    DragonLevel {
        health: 5700.0,
        attack_damage: 512.5,
    },
];

const DRAGON: UnitType = UnitType {
    name: "Dragon",
    housing_space: 20,
    levels: DRAGON_LEVELS.len(),
};

inventory::submit! {DRAGON}

const DRAGON_SPEED: f32 = 2.0;
const DRAGON_ATTACK_COOLDOWN: f32 = 1.25;
const DRAGON_ATTACK_RANGE: f32 = 1.0;

fn draw_dragon(id: EntityId, all_storages: &AllStoragesView, result: &mut Vec<Shape>) {
    let position = all_storages.borrow::<View<Position>>().unwrap()[id].0;

    result.push(Shape::Circle {
        x: position.x,
        y: position.y,
        radius: 0.25,
        color: ShapeColor::new(255, 0, 0),
    });
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DragonModel {
    pub level: usize,
}

impl UnitModel for DragonModel {
    fn create_unit(&self, world: &mut World, position: Vector2<f32>) -> Result<()> {
        let level = DRAGON_LEVELS
            .get(self.level)
            .context("Level out of range")?;

        create_air_unit(
            world,
            position,
            level.health,
            DRAGON_SPEED,
            DRAGON_ATTACK_RANGE,
            DRAGON_ATTACK_COOLDOWN,
            AirUnitFindTargetBehaviour {
                pattern: AttackTargetFlags::COUNTED_BUILDING,
            }
            .into(),
            MeleeAttackBehaviour {
                damage: level.attack_damage,
            }
            .into(),
            draw_dragon,
        );

        Ok(())
    }
}
