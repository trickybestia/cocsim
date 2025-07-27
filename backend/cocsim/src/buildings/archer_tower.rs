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

struct ArcherTowerLevel {
    pub health: f32,
    pub attack_damage: f32,
}

const ARCHER_TOWER_LEVELS: &[ArcherTowerLevel] = &[
    ArcherTowerLevel {
        health: 380.0,
        attack_damage: 5.5,
    },
    ArcherTowerLevel {
        health: 420.0,
        attack_damage: 7.5,
    },
    ArcherTowerLevel {
        health: 460.0,
        attack_damage: 9.5,
    },
    ArcherTowerLevel {
        health: 500.0,
        attack_damage: 12.5,
    },
    ArcherTowerLevel {
        health: 540.0,
        attack_damage: 15.0,
    },
    ArcherTowerLevel {
        health: 580.0,
        attack_damage: 17.5,
    },
    ArcherTowerLevel {
        health: 630.0,
        attack_damage: 21.0,
    },
    ArcherTowerLevel {
        health: 690.0,
        attack_damage: 24.0,
    },
    ArcherTowerLevel {
        health: 750.0,
        attack_damage: 28.0,
    },
    ArcherTowerLevel {
        health: 810.0,
        attack_damage: 31.5,
    },
    ArcherTowerLevel {
        health: 890.0,
        attack_damage: 35.0,
    },
    ArcherTowerLevel {
        health: 970.0,
        attack_damage: 37.0,
    },
    ArcherTowerLevel {
        health: 1050.0,
        attack_damage: 39.0,
    },
    ArcherTowerLevel {
        health: 1130.0,
        attack_damage: 41.0,
    },
    ArcherTowerLevel {
        health: 1230.0,
        attack_damage: 42.5,
    },
    ArcherTowerLevel {
        health: 1310.0,
        attack_damage: 45.0,
    },
    ArcherTowerLevel {
        health: 1390.0,
        attack_damage: 50.0,
    },
    ArcherTowerLevel {
        health: 1510.0,
        attack_damage: 55.0,
    },
    ArcherTowerLevel {
        health: 1600.0,
        attack_damage: 60.0,
    },
    ArcherTowerLevel {
        health: 1700.0,
        attack_damage: 67.5,
    },
    ArcherTowerLevel {
        health: 1800.0,
        attack_damage: 72.5,
    },
];

const ARCHER_TOWER: BuildingType = BuildingType {
    name: "ArcherTower",
    size: Vector2::new(3, 3),
    levels: ARCHER_TOWER_LEVELS.len(),
    options: &[],
};

inventory::submit! {ARCHER_TOWER}

#[derive(Serialize, Deserialize, Debug, Arbitrary)]
pub struct ArcherTowerModel {
    pub x: usize,
    pub y: usize,
    pub level: usize,
}

impl BuildingModel for ArcherTowerModel {
    fn r#type(&self) -> &'static BuildingType {
        &ARCHER_TOWER
    }

    fn position(&self) -> Vector2<usize> {
        Vector2::new(self.x, self.y)
    }

    fn validate(&self) -> anyhow::Result<()> {
        ensure!(self.level < ARCHER_TOWER_LEVELS.len());

        Ok(())
    }

    fn create_building(&self, world: &mut World) {
        create_passive_building(
            world,
            ARCHER_TOWER_LEVELS[self.level].health,
            Vector2::new(self.x, self.y),
            ARCHER_TOWER.size,
        );
    }
}
