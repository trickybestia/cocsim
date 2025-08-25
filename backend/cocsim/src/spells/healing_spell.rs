use arbitrary::Arbitrary;
use hecs::{
    Entity,
    PreparedQuery,
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
        health::Health,
        position::Position,
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

struct HealingSpellLevel {
    pub healing_per_tick: f32,
}

const HEALING_SPELL_LEVELS_LEN: usize = 11;
const HEALING_SPELL_LEVEL_INDEX_MAX: usize = HEALING_SPELL_LEVELS_LEN - 1;
const HEALING_SPELL_LEVELS: [HealingSpellLevel; HEALING_SPELL_LEVELS_LEN] = [
    HealingSpellLevel {
        healing_per_tick: 15.0,
    },
    HealingSpellLevel {
        healing_per_tick: 20.0,
    },
    HealingSpellLevel {
        healing_per_tick: 25.0,
    },
    HealingSpellLevel {
        healing_per_tick: 30.0,
    },
    HealingSpellLevel {
        healing_per_tick: 35.0,
    },
    HealingSpellLevel {
        healing_per_tick: 40.0,
    },
    HealingSpellLevel {
        healing_per_tick: 45.0,
    },
    HealingSpellLevel {
        healing_per_tick: 50.0,
    },
    HealingSpellLevel {
        healing_per_tick: 55.0,
    },
    HealingSpellLevel {
        healing_per_tick: 60.0,
    },
    HealingSpellLevel {
        healing_per_tick: 65.0,
    },
];

const HEALING_SPELL: SpellType = SpellType {
    name: "Healing",
    housing_space: 2,
    levels: HEALING_SPELL_LEVELS.len(),
};

inventory::submit! {HEALING_SPELL}

const HEALING_SPELL_RADIUS: f32 = 4.0;
const HEALING_SPELL_DURATION: f32 = 12.0;
const HEALING_SPELL_TIME_PER_TICK: f32 = 0.3;
const HEALING_SPELL_COLOR: ShapeColor = ShapeColor::new(255, 255, 0); // yellow

#[derive(Serialize, Deserialize, Debug, Clone, Arbitrary)]
pub struct HealingSpellModel {
    pub level: UsizeWithMax<HEALING_SPELL_LEVEL_INDEX_MAX>,
}

impl SpellModel for HealingSpellModel {
    fn r#type(&self) -> &'static SpellType {
        &HEALING_SPELL
    }

    fn level(&self) -> usize {
        *self.level
    }

    fn spawn(&self, game: &mut Game, position: Vector2<f32>) {
        spawn_spell(
            &mut game.world,
            position,
            Box::new(HealingSpellDrop {
                position,
                level: self.level,
            }),
            Drawable::Shapes(vec![Shape::Rect {
                x: 0.0,
                y: 0.0,
                width: 0.4,
                height: 0.4,
                color: HEALING_SPELL_COLOR,
            }]),
        );
    }
}

#[derive(Clone, Debug)]
struct HealingSpellDrop {
    pub position: Vector2<f32>,
    pub level: UsizeWithMax<HEALING_SPELL_LEVEL_INDEX_MAX>,
}

impl Action for HealingSpellDrop {
    fn call(&self, _actor: Entity, game: &mut Game) {
        let level = &HEALING_SPELL_LEVELS[*self.level];

        game.world.spawn((
            Position(self.position),
            TickSpell {
                remaining_ticks: (HEALING_SPELL_DURATION / HEALING_SPELL_TIME_PER_TICK).round()
                    as usize, // maybe add +1 here like in game
                time_per_tick: HEALING_SPELL_TIME_PER_TICK,
                remaining_time_to_next_tick: 0.0,
                tick: Box::new(HealingSpellTick {
                    healing_per_tick: level.healing_per_tick,
                }),
            },
            Drawable::Shapes(vec![Shape::Arc {
                x: 0.0,
                y: 0.0,
                radius: HEALING_SPELL_RADIUS,
                rotation: 0.0,
                angle: 360.0,
                width: 0.1,
                color: HEALING_SPELL_COLOR,
            }]),
        ));
    }
}

#[derive(Clone, Debug)]
struct HealingSpellTick {
    pub healing_per_tick: f32,
}

impl Action for HealingSpellTick {
    fn call(&self, actor: Entity, game: &mut Game) {
        let position = game.world.get::<&Position>(actor).unwrap().0;
        let spell_collider = CircleCollider::new(position, HEALING_SPELL_RADIUS);

        for (_, (target_health, target_position, target_team)) in game
            .cache
            .get_mut_or_default::<PreparedQuery<(&mut Health, &Position, &Team)>>()
            .query_mut(&mut game.world)
        {
            if *target_team == Team::Defense {
                continue;
            }

            if spell_collider.contains(target_position.0) {
                target_health.incoming_damage -= self.healing_per_tick;
            }
        }
    }
}
