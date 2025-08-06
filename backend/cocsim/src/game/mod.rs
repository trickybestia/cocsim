pub mod features;

use anymap::AnyMap;
use hecs::{
    PreparedQuery,
    World,
};
use nalgebra::Vector2;
use rand_pcg::Pcg64Mcg;

use crate::{
    BuildingModel,
    Map,
    Shape,
    UnitModel,
    UnitModelEnum,
    consts::{
        COLLISION_TILE_COLOR,
        COLLISION_TILE_SIZE,
        MAX_ATTACK_DURATION,
    },
    game::features::{
        attack::Team,
        buildings::{
            BuildingsGrid,
            CountedBuilding,
            DropZone,
            TownHall,
        },
        collision::PathfindingCollisionGrid,
        map_size::MapSize,
    },
    utils::{
        AnyMapExt,
        draw_bool_grid,
        get_tile_color,
    },
};

pub struct Game {
    pub(crate) world: World,
    pub(crate) cache: AnyMap,

    pub(crate) map_size: MapSize,
    pub(crate) rng: Pcg64Mcg,
    pub(crate) buildings_grid: BuildingsGrid,
    pub(crate) drop_zone: DropZone,
    pub(crate) collision_grid: Option<PathfindingCollisionGrid>,

    pub(crate) time_elapsed: f32,
    pub(crate) delta_time: f32,

    pub(crate) need_redraw_collision: bool,

    pub(crate) initial_counted_buildings_count: usize,
}

impl Game {
    pub fn time_elapsed(&self) -> f32 {
        self.time_elapsed
    }

    pub fn time_left(&self) -> f32 {
        MAX_ATTACK_DURATION - self.time_elapsed
    }

    pub fn map_size(&self) -> &MapSize {
        &self.map_size
    }

    pub fn drop_zone(&self) -> &DropZone {
        &self.drop_zone
    }

    pub fn done(&mut self) -> bool {
        self.time_elapsed == MAX_ATTACK_DURATION || self.stars() == 3
    }

    pub fn percentage_destroyed(&mut self) -> f32 {
        self.destroyed_counted_buildings_count() as f32 * 100.0
            / self.initial_counted_buildings_count as f32
    }

    /// Check if there is any entity with Team::Attack.
    pub fn is_attacker_team_present(&mut self) -> bool {
        for (_, team) in self
            .cache
            .get_mut_or_default::<PreparedQuery<&Team>>()
            .query_mut(&mut self.world)
        {
            if *team == Team::Attack {
                return true;
            }
        }

        false
    }

    pub fn stars(&mut self) -> u32 {
        let destroyed_buildings_count = self.destroyed_counted_buildings_count();

        let half_buildings_destroyed = (destroyed_buildings_count as f32 * 100.0
            / self.initial_counted_buildings_count as f32)
            .round()
            >= 50.0;
        let all_buildings_destroyed =
            destroyed_buildings_count == self.initial_counted_buildings_count;
        let townhall_destroyed = self
            .cache
            .get_mut_or_default::<PreparedQuery<&TownHall>>()
            .query_mut(&mut self.world)
            .into_iter()
            .count()
            == 0;

        townhall_destroyed as u32 + half_buildings_destroyed as u32 + all_buildings_destroyed as u32
    }

    pub fn progress_info(&mut self) -> String {
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
        self.need_redraw_collision
    }

    pub fn new(map: &Map, enable_collision_grid: bool, rng: Option<Pcg64Mcg>) -> Self {
        let mut world = World::new();
        let mut cache = AnyMap::new();

        let map_size = MapSize {
            base_size: map.base_size as i32,
            border_size: map.border_size as i32,
        };

        let rng = rng.unwrap_or(Pcg64Mcg::new(rand::random()));

        for building in &map.buildings {
            building.spawn(&mut world);
        }

        let initial_counted_buildings_count = Self::counted_buildings_count(&mut cache, &mut world);

        let buildings_grid = BuildingsGrid::new(&map_size, &mut world);
        let drop_zone = DropZone::new(&map_size, &buildings_grid);
        let collision_grid =
            enable_collision_grid.then(|| PathfindingCollisionGrid::new(&map_size, &world));

        let mut result = Self {
            world,
            cache,

            map_size,
            rng,
            buildings_grid,
            drop_zone,
            collision_grid,

            time_elapsed: 0.0,
            delta_time: 0.0,

            need_redraw_collision: true,

            initial_counted_buildings_count,
        };

        result.tick_cleanup();

        result
    }

    pub fn spawn_attack_unit(&mut self, model: &UnitModelEnum, position: Vector2<f32>) {
        model.spawn(&mut self.world, position, Team::Attack);
    }

    pub fn tick(&mut self, delta_time: f32) {
        self.delta_time = delta_time;

        features::attack::check_retarget(self);
        features::attack::targeting::handle_retarget(self);
        features::attack::attack(self);
        features::projectiles::target_projectile::update(self);
        features::projectiles::splash_projectile::update(self);
        features::stunned::clear(self);
        features::projectiles::air_sweeper_projectile::update(self);
        features::waypoint_mover::r#move(self);
        features::health::handle_splash_damage_events(self);
        features::health::handle_entity_damage_events(self);
        // TODO: run system: remove DeathRequest and use hero ability if not used
        features::delay::update(self);

        if self.collision_grid.is_some() {
            features::collision::check_need_redraw_collision(self);
        }

        features::to_be_deleted::handle_to_be_deleted(self);

        self.tick_cleanup();

        self.time_elapsed = MAX_ATTACK_DURATION.min(self.time_elapsed + self.delta_time);
    }

    pub fn draw_entities(&mut self) -> Vec<Shape> {
        let mut result = Vec::new();

        features::drawable::draw(&mut result, self);
        features::projectiles::target_projectile::draw(&mut result, self);
        features::projectiles::splash_projectile::draw(&mut result, self);
        features::projectiles::air_sweeper_projectile::draw(&mut result, self);

        result
    }

    pub fn draw_grid(&self) -> Vec<Shape> {
        let mut result = Vec::new();

        for x in 0..self.map_size.total_size() {
            for y in 0..self.map_size.total_size() {
                result.push(Shape::Rect {
                    x: x as f32,
                    y: y as f32,
                    width: 1.0,
                    height: 1.0,
                    color: get_tile_color(
                        (y ^ x) % 2 == 0,
                        self.map_size.is_border(Vector2::new(x, y)),
                        self.drop_zone.0[(x as usize, y as usize)],
                        self.world
                            .contains(self.buildings_grid.0[(x as usize, y as usize)]),
                    ),
                });
            }
        }

        result
    }

    pub fn draw_collision(&mut self) -> Vec<Shape> {
        match &self.collision_grid {
            Some(collision_grid) => {
                self.need_redraw_collision = false;

                draw_bool_grid(
                    collision_grid
                        .0
                        .map(|building_id| self.world.contains(building_id)),
                    COLLISION_TILE_SIZE,
                    COLLISION_TILE_COLOR,
                )
            }
            None => Vec::new(),
        }
    }

    fn tick_cleanup(&mut self) {
        //features::buildings::cleanup_tracking(self);
        //features::collision::cleanup_tracking(self);

        features::health::cleanup_events(self);
    }

    fn counted_buildings_count(cache: &mut AnyMap, world: &mut World) -> usize {
        cache
            .get_mut_or_default::<PreparedQuery<&CountedBuilding>>()
            .query_mut(world)
            .into_iter()
            .count()
    }

    fn destroyed_counted_buildings_count(&mut self) -> usize {
        self.initial_counted_buildings_count
            - Self::counted_buildings_count(&mut self.cache, &mut self.world)
    }
}
