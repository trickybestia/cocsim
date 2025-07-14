use std::ops::Deref;

use nalgebra::Vector2;

use crate::{
    Building,
    BuildingType,
    Game,
    colliders::Collider,
};

pub(crate) struct PassiveBuilding {
    pub type_: &'static BuildingType,
    pub position: Vector2<usize>,
    pub health: f32,
    pub collider: Option<Box<dyn Collider>>,
    pub on_destroyed: Vec<Box<dyn Fn(&mut Game, u32)>>,
}

impl Deref for PassiveBuilding {
    type Target = &'static BuildingType;

    fn deref(&self) -> &Self::Target {
        &self.type_
    }
}

impl Building for PassiveBuilding {
    fn position(&self) -> Vector2<usize> {
        self.position
    }

    fn health(&self) -> f32 {
        self.health
    }

    fn collider(&self) -> Option<&dyn Collider> {
        match &self.collider {
            Some(collider) => Some(collider.as_ref()),
            None => None,
        }
    }

    fn on_destroyed_mut(&mut self) -> &mut Vec<Box<dyn Fn(&mut Game, u32)>> {
        &mut self.on_destroyed
    }

    fn apply_damage(&mut self, damage: f32) {
        assert!(!self.destroyed());

        self.health = 0.0f32.max(self.health - damage);

        if self.destroyed() {
            for handler in &self.on_destroyed {
                handler()
            }
        }
    }
}
