use anyhow::ensure;
use arbitrary::Arbitrary;
use nalgebra::DMatrix;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    BuildingModel,
    BuildingModelEnum,
};

#[derive(Serialize, Deserialize, Debug, Arbitrary)]
#[serde(rename_all = "camelCase")]
pub struct Map {
    pub base_size: usize,
    pub border_size: usize,

    pub buildings: Vec<BuildingModelEnum>,
}

impl Map {
    pub fn validate(&self) -> anyhow::Result<()> {
        ensure!(self.base_size >= 1 && self.base_size <= 44);
        ensure!(self.border_size <= 4);
        ensure!(self.buildings.len() <= 1000);

        let mut has_town_hall = false;
        let mut buildings_grid = DMatrix::from_element(self.base_size, self.base_size, false);

        for building in &self.buildings {
            building.validate()?;

            if let BuildingModelEnum::TownHallModel(_) = building {
                ensure!(!has_town_hall);

                has_town_hall = true;
            }

            let start_x = building.position().x;
            let start_y = building.position().y;

            ensure!(start_x >= self.border_size && start_x < self.base_size + self.border_size);
            ensure!(start_y >= self.border_size && start_y < self.base_size + self.border_size);

            let end_x = start_x + building.r#type().size.x;
            let end_y = start_y + building.r#type().size.y;

            ensure!(end_x <= self.base_size + self.border_size);
            ensure!(end_y <= self.base_size + self.border_size);

            for x in start_x..end_x {
                for y in start_y..end_y {
                    let tile = &mut buildings_grid[(x - self.border_size, y - self.border_size)];

                    ensure!(!*tile);

                    *tile = true;
                }
            }
        }

        ensure!(has_town_hall);

        Ok(())
    }
}
