use anyhow::Context;
use nalgebra::{
    DMatrix,
    Vector2,
};
use shipyard::{
    AddComponent,
    Component,
    EntityId,
    IntoIter,
    Unique,
    UniqueOrInitView,
    UniqueView,
    UniqueViewMut,
    ViewMut,
    track::InsertionAndModification,
};

use crate::game::features::{
    map::MapSize,
    position::Position,
};

pub struct Building {
    pub position: Vector2<usize>,
    pub size: Vector2<usize>,
}

impl Component for Building {
    type Tracking = InsertionAndModification;
}

/// "Counted" means that this building impacts destroyed buildings percentage.
#[derive(Component)]
pub struct CountedBuilding;

#[derive(Component)]
pub struct TownHall;

#[derive(Unique)]
pub struct BuildingsGrid(pub DMatrix<EntityId>);

impl Default for BuildingsGrid {
    fn default() -> Self {
        unimplemented!()
    }
}

#[derive(Unique)]
pub struct DropZone(pub DMatrix<bool>);

impl Default for DropZone {
    fn default() -> Self {
        unimplemented!()
    }
}

pub fn init_buildings_grid(
    map_size: UniqueView<MapSize>,
    buildings_grid: UniqueOrInitView<BuildingsGrid>,
) {
    buildings_grid
        .set(BuildingsGrid(DMatrix::from_element(
            map_size.total_size() as usize,
            map_size.total_size() as usize,
            EntityId::dead(),
        )))
        .unwrap();
}

pub fn handle_building_changes(
    mut buildings_grid: UniqueViewMut<BuildingsGrid>,
    v_building: ViewMut<Building>,
    mut v_position: ViewMut<Position>,
) -> anyhow::Result<()> {
    let modified_ids = v_building
        .modified()
        .iter()
        .with_id()
        .map(|(id, _)| id)
        .collect::<Vec<_>>();

    if !modified_ids.is_empty() {
        for item in &mut buildings_grid.0 {
            if modified_ids.contains(item) {
                *item = EntityId::dead();
            }
        }
    }

    for (id, building) in v_building.inserted_or_modified().iter().with_id() {
        v_position.add_component_unchecked(
            id,
            Position(building.position.cast() + building.size.cast() / 2.0),
        );

        for rel_x in 0..building.size.x {
            let abs_x = building.position.x + rel_x;

            for rel_y in 0..building.size.y {
                let abs_y = building.position.y + rel_y;

                *buildings_grid
                    .0
                    .get_mut((abs_x, abs_y))
                    .context("Valid buildings_grid index expected")? = id;
            }
        }
    }

    Ok(())
}

pub fn init_drop_zone(
    map_size: UniqueView<MapSize>,
    buildings_grid: UniqueView<BuildingsGrid>,
    drop_zone: UniqueOrInitView<DropZone>,
) {
    fn get_neighbors(map_size: &MapSize, x: i32, y: i32) -> Vec<(usize, usize)> {
        let mut result = Vec::new();

        for neighbor_x in (x - 1)..(x + 2) {
            for neighbor_y in (y - 1)..(y + 2) {
                if map_size.is_inside_map(Vector2::new(neighbor_x, neighbor_y)) {
                    result.push((neighbor_x as usize, neighbor_y as usize));
                }
            }
        }

        result
    }

    let mut result = DMatrix::from_element(
        map_size.total_size() as usize,
        map_size.total_size() as usize,
        true,
    );

    for x in 0..map_size.total_size() {
        for y in 0..map_size.total_size() {
            if buildings_grid.0[(x as usize, y as usize)] != EntityId::dead() {
                for neighbor in get_neighbors(&map_size, x, y) {
                    result[neighbor] = false;
                }
            }
        }
    }

    drop_zone.set(DropZone(result)).unwrap();
}

pub fn cleanup_tracking(v_building: ViewMut<Building>) {
    v_building.clear_all_inserted_and_modified();
}
