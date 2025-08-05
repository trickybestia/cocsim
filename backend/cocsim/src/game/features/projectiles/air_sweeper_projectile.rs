use std::{
    collections::HashMap,
    f32::consts::PI,
};

use hecs::{
    Entity,
    PreparedQuery,
};

use crate::{
    Game,
    Shape,
    ShapeColor,
    game::features::{
        attack::{
            AttackTarget,
            AttackTargetFlags,
            Team,
        },
        position::Position,
        stunned::Stunned,
        to_be_deleted::ToBeDeleted,
    },
    utils::{
        AnyMapExt,
        nearest_point_on_arc,
    },
};

pub struct AirSweeperProjectile {
    pub push_strength: f32,
    pub rotation: f32,
    pub start_angle: f32,
    pub radius: f32,
    pub max_radius: f32,
    pub speed: f32,
    pub applied_push_strength: HashMap<Entity, f32>,
    pub max_arc_length: f32,
}

impl AirSweeperProjectile {
    pub fn angle(&self) -> f32 {
        let angle_for_this_arc_length = self.max_arc_length * 180.0 / PI / self.radius;

        self.start_angle.min(angle_for_this_arc_length)
    }
}

#[derive(Default)]
struct UpdateCache<'a> {
    pub projectile_query: PreparedQuery<(&'a mut AirSweeperProjectile, &'a Team, &'a Position)>,
    pub target_query: PreparedQuery<(&'a AttackTarget, &'a Team, &'a mut Position)>,
}

pub fn update(game: &mut Game) {
    let cache = game.cache.get_mut_or_default::<UpdateCache>();

    let mut to_be_deleted = Vec::new();
    let mut stunned = Vec::new();

    for (id, (projectile, projectile_team, projectile_position)) in
        cache.projectile_query.query(&game.world).iter()
    {
        for (attack_target_id, (attack_target, attack_target_team, attack_target_position)) in
            cache.target_query.query(&game.world).iter()
        {
            if attack_target_team == projectile_team
                || !attack_target.flags.contains(AttackTargetFlags::UNIT)
                || !attack_target.flags.contains(AttackTargetFlags::AIR)
            {
                continue;
            }

            let distance = nearest_point_on_arc(
                attack_target_position.0,
                projectile_position.0,
                projectile.radius,
                projectile.rotation - projectile.angle() / 2.0,
                projectile.angle(),
            )
            .metric_distance(&attack_target_position.0);

            if distance < 0.1 {
                let apply_push = if let Some(applied_push_strength) =
                    projectile.applied_push_strength.get_mut(&attack_target_id)
                {
                    if *applied_push_strength < projectile.push_strength {
                        *applied_push_strength += projectile.speed * game.delta_time;

                        true
                    } else {
                        false
                    }
                } else {
                    projectile
                        .applied_push_strength
                        .insert(attack_target_id, projectile.speed * game.delta_time);

                    true
                };

                if apply_push {
                    let push = (attack_target_position.0 - projectile_position.0).normalize()
                        * projectile.speed
                        * game.delta_time;
                    attack_target_position.0 += push;

                    stunned.push(attack_target_id);
                }
            }
        }

        projectile.radius = projectile
            .max_radius
            .min(projectile.radius + projectile.speed * game.delta_time);

        if projectile.radius == projectile.max_radius {
            to_be_deleted.push(id);
        }
    }

    for id in to_be_deleted {
        game.world.insert_one(id, ToBeDeleted).unwrap();
    }

    for id in stunned {
        game.world.insert_one(id, Stunned).unwrap();
    }
}

pub fn draw(result: &mut Vec<Shape>, game: &mut Game) {
    for (_, (projectile, position)) in game
        .cache
        .get_mut_or_default::<PreparedQuery<(&AirSweeperProjectile, &Position)>>()
        .query_mut(&mut game.world)
    {
        result.push(Shape::Arc {
            x: position.0.x,
            y: position.0.y,
            radius: projectile.radius,
            rotation: projectile.rotation - projectile.angle() / 2.0,
            angle: projectile.angle(),
            width: 0.2,
            color: ShapeColor::new(255, 255, 255),
        });
    }
}
