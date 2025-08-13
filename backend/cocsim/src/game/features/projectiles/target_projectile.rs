use hecs::{
    Entity,
    PreparedQuery,
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
        to_be_despawned::ToBeDespawned,
    },
    utils::AnyMapExt,
};

pub struct TargetProjectile {
    pub damage: f32,
    pub target: Entity,
    pub relative_position: Vector2<f32>,
    pub speed: f32,
    pub remaining_time: f32,
}

pub fn update(game: &mut Game) {
    let mut to_be_despawned = Vec::new();
    let mut entity_damage_event = Vec::new();

    for (id, (projectile, position)) in game
        .cache
        .get_mut_or_default::<PreparedQuery<(&mut TargetProjectile, &mut Position)>>()
        .query(&game.world)
        .iter()
    {
        if !game.world.contains(projectile.target) {
            to_be_despawned.push(id);

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
            to_be_despawned.push(id);
        }
    }

    for id in to_be_despawned {
        game.world.insert_one(id, ToBeDespawned).unwrap();
    }

    game.world.spawn_batch(entity_damage_event);
}

pub fn draw(result: &mut Vec<Shape>, game: &mut Game) {
    for (_, position) in game
        .cache
        .get_mut_or_default::<PreparedQuery<With<&Position, &TargetProjectile>>>()
        .query_mut(&mut game.world)
    {
        result.push(Shape::Circle {
            x: position.0.x,
            y: position.0.y,
            radius: 0.15,
            color: ShapeColor::new(255, 0, 0),
        });
    }
}
