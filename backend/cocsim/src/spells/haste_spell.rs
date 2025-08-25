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
    colliders::{
        CircleCollider,
        Collider,
    },
    game::features::{
        actions::Action,
        attack::Team,
        drawable::Drawable,
        position::Position,
        speed::{
            HasteSpellSpeedModifier,
            Speed,
        },
        tick_spell::TickSpell,
    },
    spells::{
        SpellModel,
        SpellType,
        utils::spawn_spell,
    },
    usize_with_max::UsizeWithMax,
    utils::AnyMapExt,
};

struct HasteSpellLevel {
    pub speed_increase: f32,
    pub duration: f32,
}

const HASTE_SPELL_LEVELS_LEN: usize = 6;
const HASTE_SPELL_LEVEL_INDEX_MAX: usize = HASTE_SPELL_LEVELS_LEN - 1;
const HASTE_SPELL_LEVELS: [HasteSpellLevel; HASTE_SPELL_LEVELS_LEN] = [
    HasteSpellLevel {
        speed_increase: 3.5,
        duration: 10.0,
    },
    HasteSpellLevel {
        speed_increase: 4.25,
        duration: 15.0,
    },
    HasteSpellLevel {
        speed_increase: 5.0,
        duration: 20.0,
    },
    HasteSpellLevel {
        speed_increase: 5.75,
        duration: 25.0,
    },
    HasteSpellLevel {
        speed_increase: 6.5,
        duration: 30.0,
    },
    HasteSpellLevel {
        speed_increase: 7.0,
        duration: 30.0,
    },
];

const HASTE_SPELL: SpellType = SpellType {
    name: "Haste",
    housing_space: 1,
    levels: HASTE_SPELL_LEVELS.len(),
};

inventory::submit! {HASTE_SPELL}

const HASTE_SPELL_RADIUS: f32 = 4.0;
const HASTE_SPELL_TIME_PER_TICK: f32 = 0.25;
const HASTE_SPELL_BOOST_TIME: f32 = 1.0;
const HASTE_SPELL_COLOR: ShapeColor = ShapeColor::new(255, 192, 203); // pink

#[derive(Serialize, Deserialize, Debug, Clone, Arbitrary)]
pub struct HasteSpellModel {
    pub level: UsizeWithMax<HASTE_SPELL_LEVEL_INDEX_MAX>,
}

impl SpellModel for HasteSpellModel {
    fn r#type(&self) -> &'static SpellType {
        &HASTE_SPELL
    }

    fn level(&self) -> usize {
        *self.level
    }

    fn spawn(&self, game: &mut Game, position: Vector2<f32>) {
        spawn_spell(
            &mut game.world,
            position,
            Box::new(HasteSpellDrop {
                position,
                level: self.level,
            }),
            Drawable::Shapes(vec![Shape::Rect {
                x: 0.0,
                y: 0.0,
                width: 0.2,
                height: 0.5,
                color: HASTE_SPELL_COLOR,
            }]),
        );
    }
}

#[derive(Clone, Debug)]
struct HasteSpellDrop {
    pub position: Vector2<f32>,
    pub level: UsizeWithMax<HASTE_SPELL_LEVEL_INDEX_MAX>,
}

impl Action for HasteSpellDrop {
    fn call(&self, _actor: Entity, game: &mut Game) {
        let level = &HASTE_SPELL_LEVELS[*self.level];

        game.world.spawn((
            Position(self.position),
            TickSpell {
                remaining_ticks: (level.duration / HASTE_SPELL_TIME_PER_TICK).round() as usize, // maybe add +1 here like in game
                time_per_tick: HASTE_SPELL_TIME_PER_TICK,
                remaining_time_to_next_tick: 0.0,
                tick: Box::new(HasteSpellTick {
                    speed_increase: level.speed_increase,
                }),
            },
            Drawable::Shapes(vec![Shape::Arc {
                x: 0.0,
                y: 0.0,
                radius: HASTE_SPELL_RADIUS,
                rotation: 0.0,
                angle: 360.0,
                width: 0.1,
                color: HASTE_SPELL_COLOR,
            }]),
        ));
    }
}

#[derive(Clone, Debug)]
struct HasteSpellTick {
    pub speed_increase: f32,
}

impl Action for HasteSpellTick {
    fn call(&self, actor: Entity, game: &mut Game) {
        let position = game.world.get::<&Position>(actor).unwrap().0;
        let spell_collider = CircleCollider::new(position, HASTE_SPELL_RADIUS);

        let mut add_haste_speed_modifier = Vec::new();

        for (target_id, (target_position, target_team)) in game
            .cache
            .get_mut_or_default::<PreparedQuery<With<(&Position, &Team), &Speed>>>()
            .query_mut(&mut game.world)
        {
            if *target_team == Team::Defense {
                continue;
            }

            if spell_collider.contains(target_position.0) {
                add_haste_speed_modifier.push(target_id);
            }
        }

        for id in add_haste_speed_modifier {
            let insert_speed_modifier = match game.world.get::<&HasteSpellSpeedModifier>(id) {
                Ok(modifier) => self.speed_increase >= modifier.amount,
                Err(_) => true,
            };

            if insert_speed_modifier {
                game.world
                    .insert_one(
                        id,
                        HasteSpellSpeedModifier {
                            amount: self.speed_increase,
                            remaining_time: HASTE_SPELL_BOOST_TIME,
                        },
                    )
                    .unwrap();
            }
        }
    }
}
