use anyhow::ensure;
use arbitrary::Arbitrary;
use nalgebra::Vector2;
use serde::{
    Deserialize,
    Serialize,
};
use shipyard::World;

use crate::{
    BuildingModel,
    BuildingType,
    buildings::utils::passive_building::create_passive_building,
};

struct BombTowerLevel {
    pub health: f32,
    pub attack_damage: f32,
    pub death_damage: f32,
}

const BOMB_TOWER_LEVELS: &[BombTowerLevel] = &[
    BombTowerLevel {
        health: 650.0,
        attack_damage: 26.4,
        death_damage: 150.0,
    },
    BombTowerLevel {
        health: 700.0,
        attack_damage: 30.8,
        death_damage: 180.0,
    },
    BombTowerLevel {
        health: 750.0,
        attack_damage: 35.2,
        death_damage: 220.0,
    },
    BombTowerLevel {
        health: 850.0,
        attack_damage: 44.0,
        death_damage: 260.0,
    },
    BombTowerLevel {
        health: 1050.0,
        attack_damage: 52.8,
        death_damage: 300.0,
    },
    BombTowerLevel {
        health: 1300.0,
        attack_damage: 61.6,
        death_damage: 350.0,
    },
    BombTowerLevel {
        health: 1600.0,
        attack_damage: 70.4,
        death_damage: 400.0,
    },
    BombTowerLevel {
        health: 1900.0,
        attack_damage: 79.2,
        death_damage: 450.0,
    },
    BombTowerLevel {
        health: 2300.0,
        attack_damage: 92.4,
        death_damage: 500.0,
    },
    BombTowerLevel {
        health: 2500.0,
        attack_damage: 103.4,
        death_damage: 550.0,
    },
    BombTowerLevel {
        health: 2700.0,
        attack_damage: 114.4,
        death_damage: 600.0,
    },
    BombTowerLevel {
        health: 2900.0,
        attack_damage: 125.4,
        death_damage: 650.0,
    },
];

const BOMB_TOWER: BuildingType = BuildingType {
    name: "BombTower",
    size: Vector2::new(3, 3),
    levels: BOMB_TOWER_LEVELS.len(),
    options: &[],
};

inventory::submit! {BOMB_TOWER}

#[derive(Serialize, Deserialize, Debug, Arbitrary)]
pub struct BombTowerModel {
    pub x: usize,
    pub y: usize,
    pub level: usize,
}

impl BuildingModel for BombTowerModel {
    fn r#type(&self) -> &'static BuildingType {
        &BOMB_TOWER
    }

    fn position(&self) -> Vector2<usize> {
        Vector2::new(self.x, self.y)
    }

    fn validate(&self) -> anyhow::Result<()> {
        ensure!(self.level < BOMB_TOWER_LEVELS.len());

        Ok(())
    }

    fn create_building(&self, world: &mut World) {
        create_passive_building(
            world,
            BOMB_TOWER_LEVELS[self.level].health,
            Vector2::new(self.x, self.y),
            BOMB_TOWER.size,
            None,
        );
    }
}
