use std::ops::Deref;

use nalgebra::Vector2;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    BuildingType,
    BuildingData,
    BuildingModel,
    colliders::Collider,
};

struct ArmyCampLevel {
    pub health: f32,
}

const ARMY_CAMP_LEVELS: &[ArmyCampLevel] = &[
    ArmyCampLevel { health: 250.0 },
    ArmyCampLevel { health: 270.0 },
    ArmyCampLevel { health: 290.0 },
    ArmyCampLevel { health: 310.0 },
    ArmyCampLevel { health: 330.0 },
    ArmyCampLevel { health: 350.0 },
    ArmyCampLevel { health: 400.0 },
    ArmyCampLevel { health: 500.0 },
    ArmyCampLevel { health: 600.0 },
    ArmyCampLevel { health: 700.0 },
    ArmyCampLevel { health: 800.0 },
    ArmyCampLevel { health: 850.0 },
    ArmyCampLevel { health: 900.0 },
];

pub struct ArmyCampBuildingType{
    pub base: 
}



const ARMY_CAMP: &BuildingType = &BuildingType {
    name: "ArmyCamp",
    size: Vector2::new(4, 4),
    levels: ARMY_CAMP_LEVELS.len(),
    options: vec![],
    tick: None,
    draw: None,
};

#[derive(Serialize, Deserialize)]
pub struct ArmyCampModel {
    x: u32,
    y: u32,
    level: u32,
}

impl BuildingModel for ArmyCampModel {
    fn create_building(&self) -> Box<dyn BuildingData> {
        Box::new(ArmyCamp::new(self))
    }
}

pub struct ArmyCamp {}

impl ArmyCamp {
    pub fn new(model: &ArmyCampModel) -> Self {
        Self {}
    }
}

impl Deref for ArmyCamp {
    type Target = &'static BuildingType;

    fn deref(&self) -> &Self::Target {
        &ARMY_CAMP
    }
}

impl BuildingData for ArmyCamp {
    fn position(&self) -> Vector2<usize> {
        todo!()
    }

    fn health(&self) -> f32 {
        todo!()
    }

    fn collider(&self) -> Option<&dyn Collider> {
        todo!()
    }

    fn on_destroyed_mut(&mut self) -> &mut Vec<Box<dyn Fn(&mut crate::Game, usize)>> {
        todo!()
    }

    fn apply_damage(&mut self, damage: f32) {
        todo!()
    }

    fn center(&self) -> Vector2<f32> {
        self.position().cast() + self.size.cast() / 2.0
    }

    fn destroyed(&self) -> bool {
        self.health() == 0.0
    }
}
