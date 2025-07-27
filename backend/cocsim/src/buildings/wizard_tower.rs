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

struct WizardTowerLevel {
    pub health: f32,
    pub attack_damage: f32,
}

const WIZARD_TOWER_LEVELS: &[WizardTowerLevel] = &[
    WizardTowerLevel {
        health: 620.0,
        attack_damage: 14.3,
    },
    WizardTowerLevel {
        health: 650.0,
        attack_damage: 16.9,
    },
    WizardTowerLevel {
        health: 680.0,
        attack_damage: 20.8,
    },
    WizardTowerLevel {
        health: 730.0,
        attack_damage: 26.0,
    },
    WizardTowerLevel {
        health: 840.0,
        attack_damage: 31.2,
    },
    WizardTowerLevel {
        health: 960.0,
        attack_damage: 41.6,
    },
    WizardTowerLevel {
        health: 1200.0,
        attack_damage: 52.0,
    },
    WizardTowerLevel {
        health: 1440.0,
        attack_damage: 58.5,
    },
    WizardTowerLevel {
        health: 1600.0,
        attack_damage: 65.0,
    },
    WizardTowerLevel {
        health: 1900.0,
        attack_damage: 80.6,
    },
    WizardTowerLevel {
        health: 2120.0,
        attack_damage: 91.0,
    },
    WizardTowerLevel {
        health: 2240.0,
        attack_damage: 101.4,
    },
    WizardTowerLevel {
        health: 2500.0,
        attack_damage: 109.2,
    },
    WizardTowerLevel {
        health: 2800.0,
        attack_damage: 117.0,
    },
    WizardTowerLevel {
        health: 3000.0,
        attack_damage: 123.5,
    },
    WizardTowerLevel {
        health: 3150.0,
        attack_damage: 132.6,
    },
    WizardTowerLevel {
        health: 3300.0,
        attack_damage: 143.0,
    },
];

const WIZARD_TOWER: BuildingType = BuildingType {
    name: "WizardTower",
    size: Vector2::new(3, 3),
    levels: WIZARD_TOWER_LEVELS.len(),
    options: &[],
};

inventory::submit! {WIZARD_TOWER}

#[derive(Serialize, Deserialize, Debug, Arbitrary)]
pub struct WizardTowerModel {
    pub x: usize,
    pub y: usize,
    pub level: usize,
}

impl BuildingModel for WizardTowerModel {
    fn r#type(&self) -> &'static BuildingType {
        &WIZARD_TOWER
    }

    fn position(&self) -> Vector2<usize> {
        Vector2::new(self.x, self.y)
    }

    fn validate(&self) -> anyhow::Result<()> {
        ensure!(self.level < WIZARD_TOWER_LEVELS.len());

        Ok(())
    }

    fn create_building(&self, world: &mut World) {
        create_passive_building(
            world,
            WIZARD_TOWER_LEVELS[self.level].health,
            Vector2::new(self.x, self.y),
            WIZARD_TOWER.size,
        );
    }
}
