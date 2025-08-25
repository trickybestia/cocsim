use arbitrary::Arbitrary;
use hecs::{
    Entity,
    Or,
    PreparedQuery,
    Without,
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
        attack::{
            AttackTarget,
            AttackTargetFlags,
            Team,
        },
        buildings::TownHall,
        clan_castle::ClanCastle,
        delay::Delay,
        drawable::Drawable,
        health::Health,
        position::Position,
    },
    spells::{
        SpellModel,
        SpellType,
        utils::spawn_spell,
    },
    usize_with_max::UsizeWithMax,
    utils::AnyMapExt,
};

struct LightningSpellLevel {
    pub damage: f32,
}

const LIGHTNING_SPELL_LEVELS_LEN: usize = 12;
const LIGHTNING_SPELL_LEVEL_INDEX_MAX: usize = LIGHTNING_SPELL_LEVELS_LEN - 1;
const LIGHTNING_SPELL_LEVELS: [LightningSpellLevel; LIGHTNING_SPELL_LEVELS_LEN] = [
    LightningSpellLevel { damage: 150.0 },
    LightningSpellLevel { damage: 180.0 },
    LightningSpellLevel { damage: 210.0 },
    LightningSpellLevel { damage: 240.0 },
    LightningSpellLevel { damage: 270.0 },
    LightningSpellLevel { damage: 320.0 },
    LightningSpellLevel { damage: 400.0 },
    LightningSpellLevel { damage: 480.0 },
    LightningSpellLevel { damage: 560.0 },
    LightningSpellLevel { damage: 600.0 },
    LightningSpellLevel { damage: 640.0 },
    LightningSpellLevel { damage: 680.0 },
];

const LIGHTNING_SPELL: SpellType = SpellType {
    name: "Lightning",
    housing_space: 1,
    levels: LIGHTNING_SPELL_LEVELS.len(),
};

inventory::submit! {LIGHTNING_SPELL}

const LIGHTNING_SPELL_DAMAGE_RADIUS: f32 = 2.0;
const LIGHTNING_SPELL_COLOR: ShapeColor = ShapeColor::new(0, 255, 255);

#[derive(Serialize, Deserialize, Debug, Clone, Arbitrary)]
pub struct LightningSpellModel {
    pub level: UsizeWithMax<LIGHTNING_SPELL_LEVEL_INDEX_MAX>,
}

impl SpellModel for LightningSpellModel {
    fn r#type(&self) -> &'static SpellType {
        &LIGHTNING_SPELL
    }

    fn level(&self) -> usize {
        *self.level
    }

    fn spawn(&self, game: &mut Game, position: Vector2<f32>) {
        spawn_spell(
            &mut game.world,
            position,
            Box::new(LightningSpellAttack {
                position,
                damage: LIGHTNING_SPELL_LEVELS[*self.level].damage,
            }),
            Drawable::Shapes(vec![Shape::Rect {
                x: 0.0,
                y: 0.0,
                width: 0.2,
                height: 0.5,
                color: LIGHTNING_SPELL_COLOR,
            }]),
        );
    }
}

#[derive(Clone, Debug)]
struct LightningSpellAttack {
    pub position: Vector2<f32>,
    pub damage: f32,
}

impl Action for LightningSpellAttack {
    fn call(&self, _actor: Entity, game: &mut Game) {
        let spell_collider = CircleCollider::new(self.position, LIGHTNING_SPELL_DAMAGE_RADIUS);

        for (_, (attack_target, target_health, target_position, target_team)) in game
            .cache
            .get_mut_or_default::<PreparedQuery<
                Without<(&AttackTarget, &mut Health, &Position, &Team), Or<&TownHall, &ClanCastle>>,
            >>()
            .query_mut(&mut game.world)
        {
            if *target_team == Team::Attack
                || attack_target
                    .flags
                    .contains(AttackTargetFlags::RESOURCE_BUILDING)
            {
                continue;
            }

            if spell_collider.contains(
                attack_target
                    .collider
                    .translate(target_position.0)
                    .nearest_point(self.position),
            ) {
                // Actually, this damage will be applied only on next tick, because Self::call
                // is called in to_be_despawned::handle_to_be_despawned, which is executed at
                // the end of the tick. Not so big problem I guess.
                target_health.incoming_damage += self.damage;
            }
        }

        game.world.spawn((
            Delay { time_left: 0.25 },
            Drawable::Shapes(vec![Shape::Circle {
                x: self.position.x,
                y: self.position.y,
                radius: LIGHTNING_SPELL_DAMAGE_RADIUS,
                color: LIGHTNING_SPELL_COLOR,
            }]),
        ));
    }
}
