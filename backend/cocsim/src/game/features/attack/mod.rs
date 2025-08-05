use bitflags::bitflags;
use enum_dispatch::enum_dispatch;
use hecs::Entity;
use nalgebra::Vector2;
pub mod targeting;
use crate::{
    Game,
    colliders::{
        Collider,
        ColliderEnum,
    },
    consts::*,
    game::features::{
        actions::{
            Action,
            ActionEnum,
        },
        position::Position,
        stunned::Stunned,
        waypoint_mover::WaypointMover,
    },
    utils::arc_contains_angle,
};

#[enum_dispatch]
pub trait RetargetCondition {
    fn need_retarget(
        &self,
        attacker_position: Vector2<f32>,
        target_position: Vector2<f32>,
        target_collider: &ColliderEnum,
    ) -> bool;
}

pub struct BuildingRetargetCondition {
    pub min_attack_range: f32,
    pub max_attack_range: f32,
    pub rotation_angle: Option<(f32, f32)>,
}

impl RetargetCondition for BuildingRetargetCondition {
    fn need_retarget(
        &self,
        attacker_position: Vector2<f32>,
        target_position: Vector2<f32>,
        target_collider: &ColliderEnum,
    ) -> bool {
        let target_collider = target_collider.translate(target_position);

        if attacker_position.metric_distance(&target_position) < self.min_attack_range
            || !target_collider
                .attack_area(self.max_attack_range + UNIT_DISTANCE_TO_WAYPOINT_EPS)
                .contains(attacker_position)
        {
            return true;
        }

        if let Some((rotation, angle)) = self.rotation_angle {
            let target_offset = target_position - attacker_position;
            let target_angle = target_offset.y.atan2(target_offset.x).to_degrees();

            if !arc_contains_angle(rotation, angle, target_angle) {
                return true;
            }
        }

        false
    }
}

pub struct UnitRetargetCondition;

impl RetargetCondition for UnitRetargetCondition {
    fn need_retarget(
        &self,
        _attacker_position: Vector2<f32>,
        _target_position: Vector2<f32>,
        _target_collider: &ColliderEnum,
    ) -> bool {
        false
    }
}

#[enum_dispatch(RetargetCondition)]
pub enum RetargetConditionEnum {
    BuildingRetargetCondition,
    UnitRetargetCondition,
}

pub struct Attacker {
    pub attack_cooldown: f32,
    pub remaining_attack_cooldown: f32,
    pub target: Entity,
    pub retarget_condition: RetargetConditionEnum,
    pub retarget: bool,
    pub attack: ActionEnum,
}

pub struct AttackTarget {
    pub collider: ColliderEnum,
    pub flags: AttackTargetFlags,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct AttackTargetFlags: u8 {
        const BUILDING = 1;
        const UNIT = 1 << 1;

        const AIR = 1 << 2;
        const GROUND = 1 << 3;

        const COUNTED_BUILDING = 1 << 4;

        const ACTIVE_BUILDING = 1 << 5;
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Team {
    Attack,
    Defense,
}

pub fn check_retarget(game: &mut Game) {
    for (attacker_id, (attacker, stunned)) in game
        .world
        .query::<(&mut Attacker, Option<&Stunned>)>()
        .iter()
    {
        if stunned.is_some() {
            attacker.target = Entity::DANGLING;

            continue;
        }

        let retarget = if !game.world.contains(attacker.target) {
            true
        } else {
            let attacker_position = game.world.get::<&Position>(attacker_id).unwrap().0;
            let target_position = game.world.get::<&Position>(attacker.target).unwrap().0;
            let attack_target = game.world.get::<&AttackTarget>(attacker.target).unwrap();

            attacker.retarget_condition.need_retarget(
                attacker_position,
                target_position,
                &attack_target.collider,
            )
        };

        if retarget {
            attacker.target = Entity::DANGLING;
            attacker.remaining_attack_cooldown = attacker.attack_cooldown;
            attacker.retarget = true;
        }
    }
}

pub fn attack(game: &mut Game) {
    let attack_queue = create_attack_queue(game);

    for (attack, attacker_id) in attack_queue {
        attack.call(attacker_id, game);
    }
}

fn create_attack_queue(game: &mut Game) -> Vec<(ActionEnum, Entity)> {
    let mut result = Vec::new();

    for (attacker_id, (attacker, waypoint_mover)) in game
        .world
        .query::<(&mut Attacker, Option<&WaypointMover>)>()
        .iter()
    {
        if game.world.contains(attacker.target) {
            let can_attack = if let Some(waypoint_mover) = waypoint_mover {
                // attacker is unit

                waypoint_mover.waypoints.is_empty()
            } else {
                // attacker is building

                true
            };

            if can_attack {
                attacker.remaining_attack_cooldown =
                    0.0f32.max(attacker.remaining_attack_cooldown - game.delta_time);

                if attacker.remaining_attack_cooldown == 0.0 {
                    result.push((attacker.attack.clone(), attacker_id));

                    attacker.remaining_attack_cooldown = attacker.attack_cooldown;
                }
            }
        }
    }

    result
}
