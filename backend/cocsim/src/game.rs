use std::borrow::Cow;

use nalgebra::DMatrix;

use crate::{
    Building,
    Map,
    Pathfinder,
    Shape,
    consts::*,
    utils::{
        draw_bool_grid,
        get_tile_color,
        is_inside_map,
    },
};

pub struct Game {
    base_size: i32,
    border_size: i32,

    buildings: Box<[Box<dyn Building>]>,

    buildings_grid: DMatrix<Option<u32>>,
    drop_zone: DMatrix<bool>,
    collision_grid: DMatrix<Option<u32>>,

    pathfinder: Pathfinder,

    time_elapsed: f32,

    townhall_destroyed: bool,
    /// Buildings without walls.
    destroyed_buildings_count: usize,
    /// Buildings without walls.
    total_buildings_count: usize,
    need_redraw_collision: bool,
}

impl Game {
    pub fn time_left(&self) -> f32 {
        MAX_ATTACK_DURATION - self.time_elapsed
    }

    pub fn total_size(&self) -> i32 {
        self.base_size + 2 * self.border_size
    }

    pub fn done(&self) -> bool {
        self.time_elapsed == MAX_ATTACK_DURATION || self.stars() == 3
    }

    pub fn stars(&self) -> u32 {
        let half_buildings_destroyed = (self.destroyed_buildings_count as f32 * 100.0
            / self.total_buildings_count as f32)
            .round()
            >= 50.0;
        let all_buildings_destroyed = self.destroyed_buildings_count == self.total_buildings_count;

        self.townhall_destroyed as u32
            + half_buildings_destroyed as u32
            + all_buildings_destroyed as u32
    }

    pub fn progress_info(&self) -> String {
        let total_seconds = self.time_left() as u32;
        let minutes = total_seconds / 60;

        let mut result = format!(
            "{:.0} % | {} star |",
            self.destroyed_buildings_count as f32 * 100.0 / self.total_buildings_count as f32,
            self.stars()
        );

        if minutes != 0 {
            result.push_str(&format!(" {} min", minutes));
        }

        result.push_str(&format!(" {} s left", total_seconds % 60));

        result
    }

    pub fn new(map: &Map) -> Self {
        let total_size = map.base_size + 2 * map.border_size;

        let buildings = vec![].into_boxed_slice();

        let buildings_grid = Self::compute_buildings_grid(total_size, &buildings);
        let collision_grid = Self::compute_collision_grid(total_size, &buildings);
        let drop_zone = Self::compute_drop_zone(total_size, &buildings_grid);
        let total_buildings_count = Self::compute_total_buildings_count(&buildings);

        let mut result = Self {
            base_size: map.base_size as i32,
            border_size: map.border_size as i32,
            buildings,
            buildings_grid,
            drop_zone,
            collision_grid,
            pathfinder: Pathfinder,
            time_elapsed: 0.0,
            townhall_destroyed: false,
            destroyed_buildings_count: 0,
            total_buildings_count,
            need_redraw_collision: true,
        };

        for building in &mut result.buildings {
            building
                .on_destroyed_mut()
                .push(Box::new(Game::on_building_destroyed));
        }

        result
    }

    pub fn is_border(&self, x: i32, y: i32) -> bool {
        y < self.border_size
            || x < self.border_size
            || y >= self.base_size + self.border_size
            || x >= self.base_size + self.border_size
    }

    pub fn tick(&mut self, delta_t: f32) {
        assert!(!self.done());

        for i in 0..self.buildings.len() {
            if let Some(tick_fn) = self.buildings[i].tick_fn() {
                tick_fn(self, i as u32, delta_t);
            }
        }

        self.time_elapsed = MAX_ATTACK_DURATION.min(self.time_elapsed + delta_t)
    }

    pub fn draw_entities(&self) -> Vec<Shape> {
        let mut result = Vec::new();

        for i in 0..self.buildings.len() {
            if let Some(draw_fn) = self.buildings[i].draw_fn() {
                draw_fn(self, i as u32, &mut result);
            }
        }

        result
    }

    pub fn draw_grid(&self) -> Vec<Shape> {
        let mut result = Vec::new();

        for x in 0..self.total_size() {
            for y in 0..self.total_size() {
                result.push(Shape::Rect {
                    x: x as f32,
                    y: y as f32,
                    width: 1.0,
                    height: 1.0,
                    color: Cow::Borrowed(get_tile_color(
                        (y ^ x) % 2 == 0,
                        self.is_border(x, y),
                        self.drop_zone[(x as usize, y as usize)],
                        self.buildings_grid[(x as usize, y as usize)].is_some(),
                    )),
                });
            }
        }

        result
    }

    pub fn draw_collision(&mut self) -> Vec<Shape> {
        self.need_redraw_collision = false;

        draw_bool_grid(
            self.collision_grid.map(|building_id| building_id.is_some()),
            COLLISION_TILE_SIZE,
            Cow::Borrowed(COLLISION_TILE_COLOR),
        )
    }

    fn on_building_destroyed(&mut self, building_id: u32) {
        self.need_redraw_collision = true;

        let building = &self.buildings[building_id as usize];

        if building.name() == "TownHall" {
            self.townhall_destroyed = true;
        }

        if building.name() != "Wall" {
            self.destroyed_buildings_count += 1;
        }
    }

    fn compute_total_buildings_count(buildings: &[Box<dyn Building>]) -> usize {
        buildings
            .iter()
            .filter(|building| building.name() != "Wall")
            .count()
    }

    fn compute_collision_grid(
        total_size: usize,
        buildings: &[Box<dyn Building>],
    ) -> DMatrix<Option<u32>> {
        let mut result = DMatrix::from_element(
            total_size * COLLISION_TILES_PER_MAP_TILE,
            total_size * COLLISION_TILES_PER_MAP_TILE,
            None,
        );

        for i in 0..buildings.len() {
            buildings[i].update_collision(i as u32, &mut result);
        }

        result
    }

    fn compute_buildings_grid(
        total_size: usize,
        buildings: &[Box<dyn Building>],
    ) -> DMatrix<Option<u32>> {
        let mut result = DMatrix::from_element(total_size, total_size, None);

        for i in 0..buildings.len() {
            buildings[i].occupy_tiles(i as u32, &mut result);
        }

        result
    }

    fn compute_drop_zone(
        total_size: usize,
        buildings_grid: &DMatrix<Option<u32>>,
    ) -> DMatrix<bool> {
        fn get_neighbors(total_size: i32, x: i32, y: i32) -> Vec<(usize, usize)> {
            let mut result = Vec::new();

            for neighbor_x in (x - 1)..(x + 2) {
                for neighbor_y in (y - 1)..(y + 2) {
                    if is_inside_map(total_size, neighbor_x, neighbor_y) {
                        result.push((neighbor_x as usize, neighbor_y as usize));
                    }
                }
            }

            result
        }

        let mut result = DMatrix::from_element(total_size, total_size, true);

        for x in 0..total_size {
            for y in 0..total_size {
                if buildings_grid[(x, y)].is_some() {
                    for neighbor in get_neighbors(total_size as i32, x as i32, y as i32) {
                        result[neighbor] = false;
                    }
                }
            }
        }

        result
    }
}
