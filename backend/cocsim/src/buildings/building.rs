use std::{
    cell::RefCell,
    ops::Deref,
    rc::Rc,
};

use nalgebra::{
    DMatrix,
    Vector2,
};

use crate::{
    BuildingType,
    Game,
    Shape,
    colliders::Collider,
    consts::*,
};

pub struct BuildingImpl {
    pub type_: &'static BuildingType,
    pub game: Rc<Game>,
    pub position: Vector2<usize>,
    pub health: RefCell<f32>,
    pub collider: Option<Box<dyn Collider>>,
    pub on_destroyed: RefCell<Vec<Box<dyn FnOnce(Rc<dyn Building>)>>>,
}

impl BuildingImpl {
    fn center(self: Rc<Self>) -> Vector2<f32> {
        self.position.cast() + self.size.cast() / 2.0
    }

    fn destroyed(self: Rc<Self>) -> bool {
        *self.health.borrow() == 0.0
    }

    /// Apply damage to this building. Called by units when they attack.
    fn apply_damage(self: Rc<Self>, damage: f32) {
        assert!(!self.clone().destroyed());

        let mut health = self.health.borrow_mut();

        *health = 0.0f32.max(*health - damage);

        if *self.health.borrow() == 0.0 {
            for on_destroyed_handler in self.on_destroyed.borrow_mut().drain(..) {
                //on_destroyed_handler(self as Rc<dyn Building>)
            }
        }
    }
}

impl Deref for BuildingImpl {
    type Target = BuildingImpl;

    fn deref(&self) -> &Self::Target {
        &self
    }
}

impl Building for BuildingImpl {
    fn building(&self) -> &BuildingImpl {
        &self
    }
}

pub trait Building
where
    Self: Deref<Target = BuildingImpl>,
{
    fn building(&self) -> &BuildingImpl;

    fn tick(self: Rc<Self>, game: &mut Game, delta_t: f32) {}

    fn draw(self: Rc<Self>, game: &Game, shapes: &mut Vec<Shape>) {}

    /// Occupy tiles for troops drop zone calculation. Called once.
    fn occupy_tiles(self: Rc<Self>, buildings_grid: &mut DMatrix<Option<Rc<dyn Building>>>) {
        for rel_x in 0..self.size.x {
            let abs_x = self.position.x + rel_x;

            for rel_y in 0..self.size.y {
                let abs_y = self.position.y + rel_y;

                buildings_grid[(abs_x, abs_y)] = None; // Some(self.clone() as Rc<dyn Building>)
            }
        }
    }

    /// Update collision for this building. Can be called multiple times. Need
    /// check for self.destroyed.
    fn update_collision(self: Rc<Self>, collision_grid: &mut DMatrix<Option<Rc<dyn Building>>>) {
        if let Some(collider) = &self.collider {
            for rel_x in 0..(self.size.x * COLLISION_TILES_PER_MAP_TILE) {
                let abs_x = self.position.x * COLLISION_TILES_PER_MAP_TILE + rel_x;

                for rel_y in 0..self.size.y {
                    let abs_y = self.position.y * COLLISION_TILES_PER_MAP_TILE + rel_y;

                    let occupy_tile = !self.destroyed()
                        && collider.contains(Vector2::new(
                            abs_x as f32 / COLLISION_TILES_PER_MAP_TILE as f32,
                            abs_y as f32 / COLLISION_TILES_PER_MAP_TILE as f32,
                        ));

                    collision_grid[(abs_x, abs_y)] = if occupy_tile { None } else { None }
                }
            }
        }
    }
}
