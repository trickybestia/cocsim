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
    game::features::map_size::MapSize,
};

#[derive(Serialize, Deserialize, Debug, Arbitrary, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Map {
    pub base_size: usize,
    pub border_size: usize,

    pub buildings: Vec<Building>,
}

#[derive(Clone, Debug)]
pub struct ValidatedMap {
    map: Map,
    drop_zone: DMatrix<bool>,
    drop_zone_free_tiles: Vec<(usize, usize)>,
}

impl ValidatedMap {
    pub fn drop_zone(&self) -> &DMatrix<bool> {
        &self.drop_zone
    }

    pub fn map_size(&self) -> MapSize {
        MapSize {
            base_size: self.base_size as i32,
            border_size: self.border_size as i32,
        }
    }

    pub fn drop_zone_free_tiles(&self) -> &[(usize, usize)] {
        &self.drop_zone_free_tiles
    }
}

impl TryFrom<Map> for ValidatedMap {
    type Error = anyhow::Error;

    fn try_from(value: Map) -> anyhow::Result<Self> {
        ensure!(value.base_size >= MIN_BASE_SIZE && value.base_size <= MAX_BASE_SIZE);
        ensure!(value.border_size >= MIN_BORDER_SIZE && value.border_size <= MAX_BORDER_SIZE);
        ensure!(value.buildings.len() <= MAX_BUILDINGS_COUNT);

        let map_size = MapSize {
            base_size: value.base_size as i32,
            border_size: value.border_size as i32,
        };

        let mut has_town_hall = false;
        let mut buildings_grid = DMatrix::from_element(
            map_size.total_size() as usize,
            map_size.total_size() as usize,
            false,
        );
        let mut affects_drop_zone = DMatrix::from_element(
            map_size.total_size() as usize,
            map_size.total_size() as usize,
            false,
        );

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
                    let tile = &mut buildings_grid[(x, y)];

                    ensure!(!*tile);

                    *tile = true;
                    affects_drop_zone[(x, y)] = building.model.r#type().affects_drop_zone;
                }
            }
        }

        ensure!(has_town_hall);

        let mut drop_zone = DMatrix::from_element(
            map_size.total_size() as usize,
            map_size.total_size() as usize,
            true,
        );

        for x in 0..map_size.total_size() {
            for y in 0..map_size.total_size() {
                if affects_drop_zone[(x as usize, y as usize)] {
                    for neighbor in map_size.get_neighbors(x, y) {
                        drop_zone[neighbor] = false;
                    }
                }
            }
        }

        let mut drop_zone_free_tiles = Vec::new();

        for x in 0..map_size.total_size() {
            for y in 0..map_size.total_size() {
                if drop_zone[(x as usize, y as usize)] {
                    drop_zone_free_tiles.push((x as usize, y as usize));
                }
            }
        }

        Ok(Self {
            map: value,
            drop_zone,
            drop_zone_free_tiles,
        })
    }
}

impl Deref for ValidatedMap {
    type Target = Map;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}
