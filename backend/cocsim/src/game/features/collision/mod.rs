mod colliders;

pub use colliders::*;
use nalgebra::DMatrix;
use shipyard::{
    Component,
    EntityId,
    Unique,
};

#[derive(Component)]
pub struct ColliderComponent(pub ColliderEnum);

#[derive(Unique)]
pub struct CollisionGrid(pub DMatrix<Option<EntityId>>);

#[derive(Unique)]
pub struct NeedRedrawCollision(pub bool);
