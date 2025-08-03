use shipyard::{
    AddComponent,
    Component,
    IntoIter,
    UniqueView,
    View,
    ViewMut,
};

use crate::{
    Shape,
    ShapeColor,
    game::features::{
        position::Position,
        time::Time,
        to_be_deleted::ToBeDeleted,
    },
};

#[derive(Component)]
pub struct AirSweeperProjectile {
    pub push_strength: f32,
    pub rotation: f32,
    pub angle: f32,
    pub radius: f32,
    pub max_radius: f32,
    pub speed: f32,
}

pub fn update(
    time: UniqueView<Time>,
    mut v_air_sweeper_projectile: ViewMut<AirSweeperProjectile>,
    mut v_to_be_deleted: ViewMut<ToBeDeleted>,
) {
    for (id, air_sweeper_projectile) in (&mut v_air_sweeper_projectile).iter().with_id() {
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
