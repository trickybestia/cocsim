use nalgebra::Vector2;
use shipyard::{
    AddComponent,
    Component,
    EntitiesViewMut,
    IntoIter,
    UniqueView,
    View,
    ViewMut,
};

use crate::{
    Shape,
    ShapeColor,
    game::features::{
        attack::Team,
        health::SplashDamageEvent,
        position::Position,
        time::Time,
        to_be_deleted::ToBeDeleted,
    },
};

#[derive(Component)]
pub struct SplashProjectile {
    pub damage: f32,
    pub damage_radius: f32,
    pub damage_air: bool,
    pub damage_ground: bool,
    pub target: Vector2<f32>,
    pub speed: f32,
    pub remaining_time: f32,
}

pub fn update(
    time: UniqueView<Time>,
    mut entities: EntitiesViewMut,
    mut v_splash_projectile: ViewMut<SplashProjectile>,
    mut v_position: ViewMut<Position>,
    mut v_to_be_deleted: ViewMut<ToBeDeleted>,
    mut v_splash_damage_event: ViewMut<SplashDamageEvent>,
    v_team: View<Team>,
) {
    for (id, (splash_projectile, mut position, team)) in
        (&mut v_splash_projectile, &mut v_position, &v_team)
            .iter()
            .with_id()
    {
        let speed = (splash_projectile.target - position.0).normalize() * splash_projectile.speed;

        position.0 += speed * time.delta;

        splash_projectile.remaining_time =
            0.0f32.max(splash_projectile.remaining_time - time.delta);

        if splash_projectile.remaining_time == 0.0 {
            entities.add_entity(
                &mut v_splash_damage_event,
                SplashDamageEvent {
                    attacker_team: *team,
                    damage_air: splash_projectile.damage_air,
                    damage_ground: splash_projectile.damage_ground,
                    target: splash_projectile.target,
                    damage: splash_projectile.damage,
                    radius: splash_projectile.damage_radius,
                },
            );
            v_to_be_deleted.add_component_unchecked(id, ToBeDeleted);
        }
    }
}

pub fn draw(
    result: &mut Vec<Shape>,
    v_splash_projectile: View<SplashProjectile>,
    v_position: View<Position>,
) {
    for (_, position) in (&v_splash_projectile, &v_position).iter() {
        result.push(Shape::Circle {
            x: position.0.x,
            y: position.0.y,
            radius: 0.15,
            color: ShapeColor::new(255, 0, 0),
        });
    }
}
