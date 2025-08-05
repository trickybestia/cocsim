use hecs::With;
use nalgebra::Vector2;

use crate::{
    Game,
    Shape,
    ShapeColor,
    game::features::{
        attack::Team,
        health::SplashDamageEvent,
        position::Position,
        to_be_deleted::ToBeDeleted,
    },
};

pub struct SplashProjectile {
    pub damage: f32,
    pub damage_radius: f32,
    pub damage_air: bool,
    pub damage_ground: bool,
    pub target: Vector2<f32>,
    pub speed: f32,
    pub remaining_time: f32,
}

pub fn update(game: &mut Game) {
    let mut to_be_deleted = Vec::new();
    let mut splash_damage_event = Vec::new();

    for (id, (projectile, position, team)) in game
        .world
        .query::<(&mut SplashProjectile, &mut Position, &Team)>()
        .iter()
    {
        let speed = (projectile.target - position.0).normalize() * projectile.speed;

        position.0 += speed * game.delta_time;

        projectile.remaining_time = 0.0f32.max(projectile.remaining_time - game.delta_time);

        if projectile.remaining_time == 0.0 {
            splash_damage_event.push((SplashDamageEvent {
                attacker_team: *team,
                damage_air: projectile.damage_air,
                damage_ground: projectile.damage_ground,
                target: projectile.target,
                damage: projectile.damage,
                radius: projectile.damage_radius,
            },));
            to_be_deleted.push(id);
        }
    }

    for id in to_be_deleted {
        game.world.insert_one(id, ToBeDeleted).unwrap();
    }

    game.world.spawn_batch(splash_damage_event);
}

pub fn draw(result: &mut Vec<Shape>, game: &Game) {
    for (_, position) in game
        .world
        .query::<With<&Position, &SplashProjectile>>()
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
