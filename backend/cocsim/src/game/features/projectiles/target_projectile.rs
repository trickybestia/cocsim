use nalgebra::Vector2;
use shipyard::{
    AddComponent,
    Component,
    EntitiesViewMut,
    EntityId,
    IntoIter,
    UniqueView,
    View,
    ViewMut,
};

use crate::{
    Shape,
    ShapeColor,
    consts::PROJECTILE_DISTANCE_TO_TARGET_EPS,
    game::features::{
        health::DamageEvent,
        position::Position,
        time::Time,
        to_be_deleted::ToBeDeleted,
    },
};

#[derive(Component)]
pub struct TargetProjectile {
    pub damage: f32,
    pub target: EntityId,
    pub relative_position: Vector2<f32>,
    pub speed: f32,
}

pub fn update(
    time: UniqueView<Time>,
    mut entities: EntitiesViewMut,
    mut v_target_projectile: ViewMut<TargetProjectile>,
    mut v_position: ViewMut<Position>,
    mut v_to_be_deleted: ViewMut<ToBeDeleted>,
    mut v_damage_event: ViewMut<DamageEvent>,
) {
    for (id, target_projectile) in (&mut v_target_projectile).iter().with_id() {
        if !entities.is_alive(target_projectile.target) {
            v_to_be_deleted.add_component_unchecked(id, ToBeDeleted);

            continue;
        }

        let relative_speed =
            -target_projectile.relative_position.normalize() * target_projectile.speed;

        target_projectile.relative_position += relative_speed * time.delta;

        v_position[id].0 =
            v_position[target_projectile.target].0 + target_projectile.relative_position;

        if target_projectile.relative_position.norm() <= PROJECTILE_DISTANCE_TO_TARGET_EPS {
            entities.add_entity(
                &mut v_damage_event,
                DamageEvent {
                    target: target_projectile.target,
                    damage: target_projectile.damage,
                },
            );
            v_to_be_deleted.add_component_unchecked(id, ToBeDeleted);
        }
    }
}

pub fn draw(
    result: &mut Vec<Shape>,
    v_target_projectile: View<TargetProjectile>,
    v_position: View<Position>,
) {
    for (_, position) in (&v_target_projectile, &v_position).iter() {
        result.push(Shape::Circle {
            x: position.0.x,
            y: position.0.y,
            radius: 0.15,
            color: ShapeColor::new(255, 0, 0),
        });
    }
}
