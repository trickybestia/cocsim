use std::{
    borrow::Cow,
    cell::RefCell,
    rc::Rc,
};

use nalgebra::DMatrix;

use crate::{
    Building,
    Map,
    Pathfinder,
    Shape,
    consts::*,
    utils::get_tile_color,
};

pub struct Game {
    base_size: i32,
    border_size: i32,

    buildings: Vec<Rc<RefCell<dyn Building>>>,

    buildings_grid: DMatrix<Option<u32>>,
    drop_zone: DMatrix<bool>,
    collision_grid: DMatrix<Option<u32>>,

    pathfinder: Pathfinder,

    time_elapsed: f32,

    townhall_destroyed: bool,
    /// Buildings without walls.
    destroyed_buildings_count: u32,
    /// Buildings without walls.
    total_buildings_count: u32,
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
        Self {
            base_size: map.base_size as i32,
            border_size: map.border_size as i32,
            buildings: todo!(),
            buildings_grid: todo!(),
            drop_zone: todo!(),
            collision_grid: todo!(),
            pathfinder: Pathfinder,
            time_elapsed: 0.0,
            townhall_destroyed: false,
            destroyed_buildings_count: 0,
            total_buildings_count: todo!(),
            need_redraw_collision: true,
        }
    }

    pub fn is_border(&self, x: i32, y: i32) -> bool {
        y < self.border_size
            || x < self.border_size
            || y >= self.base_size + self.border_size
            || x >= self.base_size + self.border_size
    }

    pub fn is_inside_map(&self, x: i32, y: i32) -> bool {
        0 <= x && x < self.total_size() && 0 <= y && y < self.total_size()
    }

    pub fn tick(&mut self, delta_t: f32) {
        assert!(!self.done());

        for i in 0..self.buildings.len() {
            let building = self.buildings[i].clone();

            building.borrow_mut().tick(self, delta_t);
        }

        self.time_elapsed = MAX_ATTACK_DURATION.min(self.time_elapsed + delta_t)
    }

    pub fn draw_entities(&self) -> Vec<Shape> {
        let mut result = Vec::new();

        for i in 0..self.buildings.len() {
            let building = self.buildings[i].clone();

            building.borrow().draw(self, &mut result);
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
}
