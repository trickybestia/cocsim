use std::collections::HashMap;

use shipyard::{
    AddComponent,
    Component,
    EntityId,
    IntoIter,
    UniqueView,
    View,
    ViewMut,
};

use crate::{
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
        time::Time,
        to_be_deleted::ToBeDeleted,
    },
    utils::nearest_point_on_arc,
};

#[derive(Component)]
pub struct AirSweeperProjectile {
    pub push_strength: f32,
    pub rotation: f32,
    pub angle: f32,
    pub radius: f32,
    pub max_radius: f32,
    pub speed: f32,
    pub applied_push_strength: HashMap<EntityId, f32>,
}

pub fn update(
    time: UniqueView<Time>,
    v_attack_target: View<AttackTarget>,
    v_team: View<Team>,
    mut v_position: ViewMut<Position>,
    mut v_stunned: ViewMut<Stunned>,
    mut v_air_sweeper_projectile: ViewMut<AirSweeperProjectile>,
    mut v_to_be_deleted: ViewMut<ToBeDeleted>,
) {
    for (id, air_sweeper_projectile) in (&mut v_air_sweeper_projectile).iter().with_id() {
        let projectile_team = v_team[id];
        let projectile_position = v_position[id].0;

        for (attack_target_id, (attack_target, attack_target_team, mut attack_target_position)) in
            (&v_attack_target, &v_team, &mut v_position)
                .iter()
                .with_id()
        {
            if *attack_target_team == projectile_team
                || !attack_target.flags.contains(AttackTargetFlags::UNIT)
                || !attack_target.flags.contains(AttackTargetFlags::AIR)
            {
                continue;
            }

            let distance = nearest_point_on_arc(
                attack_target_position.0,
                projectile_position,
                air_sweeper_projectile.radius,
                air_sweeper_projectile.rotation,
                air_sweeper_projectile.angle,
            )
            .metric_distance(&attack_target_position.0);

            if distance < 0.1 {
                let apply_push = if let Some(applied_push_strength) = air_sweeper_projectile
                    .applied_push_strength
                    .get_mut(&attack_target_id)
                {
                    if *applied_push_strength < air_sweeper_projectile.push_strength {
                        *applied_push_strength += air_sweeper_projectile.speed * time.delta;

                        true
                    } else {
                        false
                    }
                } else {
                    air_sweeper_projectile
                        .applied_push_strength
                        .insert(attack_target_id, air_sweeper_projectile.speed * time.delta);

                    true
                };

                if apply_push {
                    let push = (attack_target_position.0 - projectile_position).normalize()
                        * air_sweeper_projectile.speed
                        * time.delta;
                    attack_target_position.0 += push;

                    v_stunned.add_component_unchecked(attack_target_id, Stunned);
                }
            }
        }

        air_sweeper_projectile.radius = air_sweeper_projectile
            .max_radius
            .min(air_sweeper_projectile.radius + air_sweeper_projectile.speed * time.delta);

        if air_sweeper_projectile.radius == air_sweeper_projectile.max_radius {
            v_to_be_deleted.add_component_unchecked(id, ToBeDeleted);
        }
    }
}

pub fn draw(
    result: &mut Vec<Shape>,
    v_air_sweeper_projectile: View<AirSweeperProjectile>,
    v_position: View<Position>,
) {
    for (air_sweeper_projectile, position) in (&v_air_sweeper_projectile, &v_position).iter() {
        result.push(Shape::Arc {
            x: position.0.x,
            y: position.0.y,
            radius: air_sweeper_projectile.radius,
            rotation: air_sweeper_projectile.rotation,
            angle: air_sweeper_projectile.angle,
            width: 0.2,
            color: ShapeColor::new(255, 255, 255),
        });
    }
}
