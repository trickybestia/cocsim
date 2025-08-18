use arbitrary::Arbitrary;
use hecs::{
    Entity,
    PreparedQuery,
    With,
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
    SpellModel,
    colliders::{
        CircleCollider,
        Collider,
    },
    game::features::{
        actions::Action,
        attack::Team,
        damage::RageSpellDamageModifier,
        drawable::Drawable,
        position::Position,
        speed::{
            RageSpellSpeedModifier,
            Speed,
        },
        tick_spell::TickSpell,
    },
    spells::{
        SpellType,
        utils::spawn_spell,
    },
    usize_with_max::UsizeWithMax,
    utils::AnyMapExt,
};

struct RageSpellLevel {
    pub damage_increase: f32,
    pub speed_increase: f32,
}

const RAGE_SPELL_LEVELS_LEN: usize = 6;
const RAGE_SPELL_LEVEL_INDEX_MAX: usize = RAGE_SPELL_LEVELS_LEN - 1;
const RAGE_SPELL_LEVELS: [RageSpellLevel; RAGE_SPELL_LEVELS_LEN] = [
    RageSpellLevel {
        damage_increase: 1.3,
        speed_increase: 2.5,
    },
    RageSpellLevel {
        damage_increase: 1.4,
        speed_increase: 2.75,
    },
    RageSpellLevel {
        damage_increase: 1.5,
        speed_increase: 3.0,
    },
    RageSpellLevel {
        damage_increase: 1.6,
        speed_increase: 3.25,
    },
    RageSpellLevel {
        damage_increase: 1.7,
        speed_increase: 3.5,
    },
    RageSpellLevel {
        damage_increase: 1.8,
        speed_increase: 3.75,
    },
];

const RAGE_SPELL: SpellType = SpellType {
    name: "Rage",
    housing_space: 2,
    levels: RAGE_SPELL_LEVELS.len(),
};

inventory::submit! {RAGE_SPELL}

const RAGE_SPELL_RADIUS: f32 = 5.0;
const RAGE_SPELL_DURATION: f32 = 18.0;
const RAGE_SPELL_TIME_PER_TICK: f32 = 0.3;
const RAGE_SPELL_BOOST_TIME: f32 = 1.0;
const RAGE_SPELL_COLOR: ShapeColor = ShapeColor::new(127, 0, 255); // violet

#[derive(Serialize, Deserialize, Debug, Clone, Arbitrary)]
pub struct RageSpellModel {
    pub level: UsizeWithMax<RAGE_SPELL_LEVEL_INDEX_MAX>,
}

impl SpellModel for RageSpellModel {
    fn r#type(&self) -> &'static SpellType {
        &RAGE_SPELL
    }

    fn level(&self) -> usize {
        *self.level
    }

    fn spawn(&self, game: &mut Game, position: Vector2<f32>) {
        spawn_spell(
            &mut game.world,
            position,
            Box::new(RageSpellDrop {
                position,
                level: self.level,
            }),
            Drawable::Shapes(vec![Shape::Rect {
                x: 0.0,
                y: 0.0,
                width: 0.4,
                height: 0.4,
                color: RAGE_SPELL_COLOR,
            }]),
        );
    }
}

#[derive(Clone, Debug)]
struct RageSpellDrop {
    pub position: Vector2<f32>,
    pub level: UsizeWithMax<RAGE_SPELL_LEVEL_INDEX_MAX>,
}

impl Action for RageSpellDrop {
    fn call(&self, _actor: Entity, game: &mut Game) {
        let level = &RAGE_SPELL_LEVELS[*self.level];

        game.world.spawn((
            Position(self.position),
            TickSpell {
                remaining_ticks: (RAGE_SPELL_DURATION / RAGE_SPELL_TIME_PER_TICK).round() as usize, // maybe add +1 here like in game
                time_per_tick: RAGE_SPELL_TIME_PER_TICK,
                remaining_time_to_next_tick: 0.0,
                tick: Box::new(RageSpellTick {
                    damage_increase: level.damage_increase,
                    speed_increase: level.speed_increase,
                }),
            },
            Drawable::Shapes(vec![Shape::Arc {
                x: 0.0,
                y: 0.0,
                radius: RAGE_SPELL_RADIUS,
                rotation: 0.0,
                angle: 360.0,
                width: 0.1,
                color: RAGE_SPELL_COLOR,
            }]),
        ));
    }
}

#[derive(Clone, Debug)]
struct RageSpellTick {
    pub damage_increase: f32,
    pub speed_increase: f32,
}

impl Action for RageSpellTick {
    fn call(&self, actor: Entity, game: &mut Game) {
        let position = game.world.get::<&Position>(actor).unwrap().0;
        let spell_collider = CircleCollider::new(position, RAGE_SPELL_RADIUS);

        let mut add_rage_modifier = Vec::new();

        for (target_id, (target_position, target_team)) in game
            .cache
            .get_mut_or_default::<PreparedQuery<With<(&Position, &Team), &Speed>>>()
            .query(&game.world)
            .iter()
        {
            if *target_team == Team::Defense {
                continue;
            }

            if spell_collider.contains(target_position.0) {
                add_rage_modifier.push(target_id);
            }
        }

        for id in add_rage_modifier {
            let insert_speed_modifier = match game.world.get::<&RageSpellSpeedModifier>(id) {
                Ok(modifier) => self.speed_increase >= modifier.amount,
                Err(_) => true,
            };

            if insert_speed_modifier {
                game.world
                    .insert_one(
                        id,
                        RageSpellSpeedModifier {
                            amount: self.speed_increase,
                            remaining_time: RAGE_SPELL_BOOST_TIME,
                        },
                    )
                    .unwrap();
            }

            let insert_damage_modifier = match game.world.get::<&RageSpellDamageModifier>(id) {
                Ok(modifier) => self.damage_increase >= modifier.amount,
                Err(_) => true,
            };

            if insert_damage_modifier {
                game.world
                    .insert_one(
                        id,
                        RageSpellDamageModifier {
                            amount: self.damage_increase,
                            remaining_time: RAGE_SPELL_BOOST_TIME,
                        },
                    )
                    .unwrap();
            }
        }
    }
}
