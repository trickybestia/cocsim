use std::borrow::Cow;

use nalgebra::{
    DMatrix,
    Vector2,
};
use shipyard::{
    Component,
    EntityId,
    IntoIter,
    Unique,
    UniqueOrInitViewMut,
    UniqueView,
    View,
    World,
    iter::WithId,
};

use crate::{
    BuildingModel,
    Map,
    Pathfinder,
    Shape,
    colliders::{
        Collider,
        ColliderEnum,
    },
    consts::*,
    utils::{
        draw_bool_grid,
        get_tile_color,
    },
};

#[derive(Unique)]
struct MapSize {
    base_size: i32,
    border_size: i32,
}

impl MapSize {
    pub fn total_size(&self) -> i32 {
        self.base_size + 2 * self.border_size
    }

    pub fn is_inside_map(&self, x: i32, y: i32) -> bool {
        0 <= x && x < self.total_size() && 0 <= y && y < self.total_size()
    }

    pub fn is_border(&self, x: i32, y: i32) -> bool {
        y < self.border_size
            || x < self.border_size
            || y >= self.base_size + self.border_size
            || x >= self.base_size + self.border_size
    }
}

#[derive(Unique)]
struct Time {
    elapsed: f32,
    delta: f32,
}

#[derive(Unique)]
struct BuildingsGrid(DMatrix<Option<EntityId>>);

#[derive(Unique)]
struct DropZone(DMatrix<bool>);

#[derive(Unique)]
struct CollisionGrid(DMatrix<Option<EntityId>>);

#[derive(Unique)]
struct NeedRedrawCollision(bool);

#[derive(Component)]
struct ColliderComponent(ColliderEnum);

#[derive(Component)]
struct Building {
    position: Vector2<usize>,
    size: Vector2<usize>,
}

/// "Counted" means that this building impacts destroyed buildings percentage.
#[derive(Component)]
struct CountedBuilding;

#[derive(Component)]
struct TownHall;

pub struct Game {
    world: World,

    initial_counted_buildings_count: usize,
}

impl Game {
    pub fn time_left(&self) -> f32 {
        MAX_ATTACK_DURATION - self.world.get_unique::<&Time>().unwrap().elapsed
    }

    pub fn done(&self) -> bool {
        self.world.get_unique::<&Time>().unwrap().elapsed == MAX_ATTACK_DURATION
            || self.stars() == 3
    }

    fn destroyed_counted_buildings_count(&self) -> usize {
        self.initial_counted_buildings_count - Self::compute_counted_buildings_count(&self.world)
    }

    pub fn stars(&self) -> u32 {
        let destroyed_buildings_count = self.destroyed_counted_buildings_count();

        let half_buildings_destroyed = (destroyed_buildings_count as f32 * 100.0
            / self.initial_counted_buildings_count as f32)
            .round()
            >= 50.0;
        let all_buildings_destroyed =
            destroyed_buildings_count == self.initial_counted_buildings_count;
        let townhall_destroyed = self.world.iter::<&TownHall>().iter().count() != 0;

        townhall_destroyed as u32 + half_buildings_destroyed as u32 + all_buildings_destroyed as u32
    }

    pub fn progress_info(&self) -> String {
        let total_seconds = self.time_left() as u32;
        let minutes = total_seconds / 60;

        let mut result = format!(
            "{:.0} % | {} star |",
            self.destroyed_counted_buildings_count() as f32 * 100.0
                / self.initial_counted_buildings_count as f32,
            self.stars()
        );

        if minutes != 0 {
            result.push_str(&format!(" {} min", minutes));
        }

        result.push_str(&format!(" {} s left", total_seconds % 60));

        result
    }

    pub fn new(map: &Map) -> Self {
        let mut world = World::new();

        world.add_unique(MapSize {
            base_size: map.base_size as i32,
            border_size: map.border_size as i32,
        });
        world.add_unique(Time {
            elapsed: 0.0,
            delta: 0.0,
        });

        for building in &map.buildings {
            building.create_building(&mut world);
        }

        let initial_counted_buildings_count = Self::compute_counted_buildings_count(&world);

        Self::create_buildings_grid(&mut world);
        Self::create_drop_zone(&mut world);
        Self::create_collision_grid(&mut world);

        Self {
            world,
            initial_counted_buildings_count,
        }
    }

    /*
    pub fn tick(&mut self, delta_t: f32) {
        assert!(!self.done());

        for i in 0..self.buildings.len() {
            if let Some(tick_fn) = self.buildings[i].tick() {
                tick_fn(self, i, delta_t);
            }
        }

        self.time_elapsed = MAX_ATTACK_DURATION.min(self.time_elapsed + delta_t)
    }

    pub fn draw_entities(&self) -> Vec<Shape> {
        let mut result = Vec::new();

        for i in 0..self.buildings.len() {
            if let Some(draw_fn) = self.buildings[i].draw() {
                draw_fn(self, i, &mut result);
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

    fn on_building_destroyed(&mut self, building_id: usize) {
        self.need_redraw_collision = true;

        let building = &self.buildings[building_id as usize];

        if building.name == "TownHall" {
            self.townhall_destroyed = true;
        }

        if building.name != "Wall" {
            self.destroyed_buildings_count += 1;
        }
    }*/

    fn compute_counted_buildings_count(world: &World) -> usize {
        world.iter::<&CountedBuilding>().iter().count()
    }

    fn create_buildings_grid(world: &mut World) {
        let map_size = world.get_unique::<&MapSize>().unwrap();

        let mut result = DMatrix::from_element(
            map_size.total_size() as usize,
            map_size.total_size() as usize,
            None,
        );

        world.run_with_data(
            |result: &mut DMatrix<Option<EntityId>>, building: View<Building>| {
                for (id, building) in building.iter().with_id() {
                    for rel_x in 0..building.size.x {
                        let abs_x = building.position.x + rel_x;

                        for rel_y in 0..building.size.y {
                            let abs_y = building.position.y + rel_y;

                            result[(abs_x, abs_y)] = Some(id)
                        }
                    }
                }
            },
            &mut result,
        );

        world.add_unique(CollisionGrid(result));
    }

    fn create_drop_zone(world: &mut World) {
        fn get_neighbors(map_size: &MapSize, x: i32, y: i32) -> Vec<(usize, usize)> {
            let mut result = Vec::new();

            for neighbor_x in (x - 1)..(x + 2) {
                for neighbor_y in (y - 1)..(y + 2) {
                    if map_size.is_inside_map(neighbor_x, neighbor_y) {
                        result.push((neighbor_x as usize, neighbor_y as usize));
                    }
                }
            }

            result
        }

        let map_size = world.get_unique::<&MapSize>().unwrap();
        let buildings_grid = world.get_unique::<&BuildingsGrid>().unwrap();

        let mut result = DMatrix::from_element(
            map_size.total_size() as usize,
            map_size.total_size() as usize,
            true,
        );

        for x in 0..map_size.total_size() {
            for y in 0..map_size.total_size() {
                if buildings_grid.0[(x as usize, y as usize)].is_some() {
                    for neighbor in get_neighbors(&map_size, x as i32, y as i32) {
                        result[neighbor] = false;
                    }
                }
            }
        }

        world.add_unique(DropZone(result));
    }

    fn create_collision_grid(world: &mut World) {
        let map_size = world.get_unique::<&MapSize>().unwrap();

        let mut result = DMatrix::from_element(
            map_size.total_size() as usize * COLLISION_TILES_PER_MAP_TILE,
            map_size.total_size() as usize * COLLISION_TILES_PER_MAP_TILE,
            None,
        );

        world.run_with_data(
            |result: &mut DMatrix<Option<EntityId>>,
             building: View<Building>,
             collider: View<ColliderComponent>| {
                for (id, (building, collider)) in (&building, &collider).iter().with_id() {
                    for rel_x in 0..(building.size.x * COLLISION_TILES_PER_MAP_TILE) {
                        let abs_x = building.position.x * COLLISION_TILES_PER_MAP_TILE + rel_x;

                        for rel_y in 0..building.size.y {
                            let abs_y = building.position.y * COLLISION_TILES_PER_MAP_TILE + rel_y;

                            let occupy_tile = collider.0.contains(Vector2::new(
                                abs_x as f32 / COLLISION_TILES_PER_MAP_TILE as f32,
                                abs_y as f32 / COLLISION_TILES_PER_MAP_TILE as f32,
                            ));

                            result[(abs_x, abs_y)] = if occupy_tile { Some(id) } else { None }
                        }
                    }
                }
            },
            &mut result,
        );

        world.add_unique(CollisionGrid(result));
    }
}
