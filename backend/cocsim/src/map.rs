use std::ops::Deref;

use anyhow::ensure;
use arbitrary::Arbitrary;
use nalgebra::DMatrix;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    Building,
    BuildingModel,
    BuildingModelEnum,
    consts::{
        MAX_BASE_SIZE,
        MAX_BORDER_SIZE,
        MAX_BUILDINGS_COUNT,
        MIN_BASE_SIZE,
        MIN_BORDER_SIZE,
    },
};

#[derive(Serialize, Deserialize, Debug, Arbitrary, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Map {
    pub base_size: usize,
    pub border_size: usize,

    pub buildings: Vec<Building>,
}

#[derive(Clone, Debug)]
pub struct ValidatedMap(Map);

impl TryFrom<Map> for ValidatedMap {
    type Error = anyhow::Error;

    fn try_from(value: Map) -> anyhow::Result<Self> {
        ensure!(value.base_size >= MIN_BASE_SIZE && value.base_size <= MAX_BASE_SIZE);
        ensure!(value.border_size >= MIN_BORDER_SIZE && value.border_size <= MAX_BORDER_SIZE);
        ensure!(value.buildings.len() <= MAX_BUILDINGS_COUNT);

        let mut has_town_hall = false;
        let mut buildings_grid = DMatrix::from_element(value.base_size, value.base_size, false);

        for building in &value.buildings {
            if let BuildingModelEnum::TownHallModel(_) = building.model {
                ensure!(!has_town_hall);

                has_town_hall = true;
            }

            let start_x = *building.x;
            let start_y = *building.y;

            ensure!(start_x >= value.border_size && start_x < value.base_size + value.border_size);
            ensure!(start_y >= value.border_size && start_y < value.base_size + value.border_size);

            let end_x = start_x + building.model.r#type().size.x;
            let end_y = start_y + building.model.r#type().size.y;

            ensure!(end_x <= value.base_size + value.border_size);
            ensure!(end_y <= value.base_size + value.border_size);

            for x in start_x..end_x {
                for y in start_y..end_y {
                    let tile = &mut buildings_grid[(x - value.border_size, y - value.border_size)];

                    ensure!(!*tile);

                    *tile = true;
                }
            }
        }

        ensure!(has_town_hall);

        Ok(Self(value))
    }
}

impl Deref for ValidatedMap {
    type Target = Map;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
