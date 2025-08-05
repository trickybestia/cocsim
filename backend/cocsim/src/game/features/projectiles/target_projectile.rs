use hecs::{
    Entity,
    With,
};
use nalgebra::Vector2;

use crate::{
    Game,
    Shape,
    ShapeColor,
    game::features::{
        health::EntityDamageEvent,
        position::Position,
        to_be_deleted::ToBeDeleted,
    },
};

pub struct TargetProjectile {
    pub damage: f32,
    pub target: Entity,
    pub relative_position: Vector2<f32>,
    pub speed: f32,
    pub remaining_time: f32,
}

pub fn update(game: &mut Game) {
    let mut to_be_deleted = Vec::new();
    let mut entity_damage_event = Vec::new();

    for (id, (projectile, position)) in game
        .world
        .query::<(&mut TargetProjectile, &mut Position)>()
        .iter()
    {
        if !game.world.contains(projectile.target) {
            to_be_deleted.push(id);

            continue;
        }

        let relative_speed = -projectile.relative_position.normalize() * projectile.speed;

        projectile.relative_position += relative_speed * game.delta_time;

        position.0 = game.world.get::<&Position>(projectile.target).unwrap().0
            + projectile.relative_position;

        projectile.remaining_time = 0.0f32.max(projectile.remaining_time - game.delta_time);

        if projectile.remaining_time == 0.0 {
            entity_damage_event.push((EntityDamageEvent {
                target: projectile.target,
                damage: projectile.damage,
            },));
            to_be_deleted.push(id);
        }
    }

    for id in to_be_deleted {
        game.world.insert_one(id, ToBeDeleted).unwrap();
    }

    game.world.spawn_batch(entity_damage_event);
}

pub fn draw(result: &mut Vec<Shape>, game: &Game) {
    for (_, position) in game
        .world
        .query::<With<&Position, &TargetProjectile>>()
        .iter()
    {
        result.push(Shape::Circle {
            x: position.0.x,
            y: position.0.y,
            radius: 0.15,
            color: ShapeColor::new(255, 0, 0),
        });
    }
}
