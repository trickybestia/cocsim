pub mod features;

use anyhow::{
    Result,
    ensure,
};
use nalgebra::Vector2;
use shipyard::{
    EntitiesView,
    UniqueView,
    World,
};

use crate::{
    BuildingModel,
    Map,
    Shape,
    UnitModel,
    UnitModelEnum,
    consts::*,
    game::features::{
        buildings::{
            BuildingsGrid,
            CountedBuilding,
            DropZone,
            TownHall,
        },
        collision::{
            NeedRedrawCollision,
            PathfindingCollisionGrid,
        },
        map::MapSize,
        time::Time,
    },
    utils::{
        draw_bool_grid,
        get_tile_color,
    },
};

pub struct Game {
    world: World,

    initial_counted_buildings_count: usize,
    enable_collision_grid: bool,
}

impl Game {
    pub fn time_elapsed(&self) -> f32 {
        self.world.get_unique::<&Time>().unwrap().elapsed
    }

    pub fn time_left(&self) -> f32 {
        MAX_ATTACK_DURATION - self.time_elapsed()
    }

    pub fn map_size(&self) -> UniqueView<MapSize> {
        self.world.get_unique::<&MapSize>().unwrap()
    }

    pub fn drop_zone(&self) -> UniqueView<DropZone> {
        self.world.get_unique::<&DropZone>().unwrap()
    }

    pub fn done(&self) -> bool {
        self.world.get_unique::<&Time>().unwrap().elapsed == MAX_ATTACK_DURATION
            || self.stars() == 3
    }

    pub fn stars(&self) -> u32 {
        let destroyed_buildings_count = self.destroyed_counted_buildings_count();

        let half_buildings_destroyed = (destroyed_buildings_count as f32 * 100.0
            / self.initial_counted_buildings_count as f32)
            .round()
            >= 50.0;
        let all_buildings_destroyed =
            destroyed_buildings_count == self.initial_counted_buildings_count;
        let townhall_destroyed = self.world.iter::<&TownHall>().iter().count() == 0;

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
            result.push_str(&format!(" {minutes} min"));
        }

        result.push_str(&format!(" {} s left", total_seconds % 60));

        result
    }

    pub fn need_redraw_collision(&self) -> bool {
        self.world.get_unique::<&NeedRedrawCollision>().unwrap().0
    }

    pub fn new(map: &Map, enable_collision_grid: bool) -> anyhow::Result<Self> {
        ensure!(
            map.base_size >= 1 && map.base_size <= 44,
            "Invalid map.base_size = {0}",
            map.base_size
        );
        ensure!(
            map.border_size <= 4,
            "Invalid map.border_size = {0}",
            map.border_size
        );

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
            building.create_building(&mut world)?;
        }

        let initial_counted_buildings_count = Self::counted_buildings_count(&world);

        world.run(features::buildings::init_buildings_grid);
        world.run(features::buildings::handle_building_changes);
        world.run(features::wall::update_walls);

        world.run(features::buildings::init_drop_zone);

        if enable_collision_grid {
            world.run(features::collision::init_collision_grid);
            world.run(features::collision::update_collision);
        }

        Self::tick_cleanup(&mut world);

        Ok(Self {
            world,
            initial_counted_buildings_count,
            enable_collision_grid,
        })
    }

    pub fn spawn_unit(&mut self, model: &UnitModelEnum, position: Vector2<f32>) -> Result<()> {
        model.create_unit(&mut self.world, position)?;

        Ok(())
    }

    pub fn tick(&mut self, delta_time: f32) {
        assert!(!self.done());

        self.world
            .run_with_data(features::time::set_delta_time, delta_time);

        self.world.run(features::attack::find_target);
        self.world.run(features::attack::attack);
        self.world.run(features::waypoint_mover::r#move);
        self.world.run(features::health::handle_damage_events);
        // TODO: run system: remove DeathRequest and use hero ability if not used
        self.world.run(features::wall::update_walls);
        self.world.run(features::health::handle_to_be_deleted);
        self.world.run(features::buildings::handle_building_changes);

        if self.enable_collision_grid {
            self.world.run(features::collision::update_collision);
        }

        Self::tick_cleanup(&mut self.world);

        self.world.run(features::time::update_elapsed_time);
    }

    pub fn draw_entities(&self) -> Vec<Shape> {
        let mut result = Vec::new();

        self.world
            .run_with_data(features::drawable::draw, &mut result);

        result
    }

    pub fn draw_grid(&self) -> Vec<Shape> {
        let map_size = self.world.get_unique::<&MapSize>().unwrap();
        let drop_zone = self.world.get_unique::<&DropZone>().unwrap();
        let buildings_grid = self.world.get_unique::<&BuildingsGrid>().unwrap();
        let entities = self.world.borrow::<EntitiesView>().unwrap();

        let mut result = Vec::new();

        for x in 0..map_size.total_size() {
            for y in 0..map_size.total_size() {
                result.push(Shape::Rect {
                    x: x as f32,
                    y: y as f32,
                    width: 1.0,
                    height: 1.0,
                    color: get_tile_color(
                        (y ^ x) % 2 == 0,
                        map_size.is_border(Vector2::new(x, y)),
                        drop_zone.0[(x as usize, y as usize)],
                        entities.is_alive(buildings_grid.0[(x as usize, y as usize)]),
                    ),
                });
            }
        }

        result
    }

    pub fn draw_collision(&mut self) -> Vec<Shape> {
        if !self.enable_collision_grid {
            return Vec::new();
        }

        let collision_grid = self
            .world
            .get_unique::<&PathfindingCollisionGrid>()
            .unwrap();
        let mut need_redraw_collision =
            self.world.get_unique::<&mut NeedRedrawCollision>().unwrap();
        let entities = self.world.borrow::<EntitiesView>().unwrap();

        need_redraw_collision.0 = false;

        draw_bool_grid(
            collision_grid
                .0
                .map(|building_id| entities.is_alive(building_id)),
            COLLISION_TILE_SIZE,
            COLLISION_TILE_COLOR,
        )
    }

    fn tick_cleanup(world: &mut World) {
        world.run(features::buildings::cleanup_tracking);
        world.run(features::collision::cleanup_tracking);
        world.run(features::position::cleanup_tracking);
        world.run(features::wall::cleanup_tracking);

        world.run(features::health::cleanup_events);
    }

    fn counted_buildings_count(world: &World) -> usize {
        world.iter::<&CountedBuilding>().iter().count()
    }

    fn destroyed_counted_buildings_count(&self) -> usize {
        self.initial_counted_buildings_count - Self::counted_buildings_count(&self.world)
    }
}
