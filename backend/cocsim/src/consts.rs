use crate::ShapeColor;

pub const TEST_IMAGES_PATH: &str = "test_images";
pub const TEST_MAPS_PATH: &str = "test_maps";

/// Used for pathfinding.
pub const COLLISION_TILES_PER_MAP_TILE: usize = 10;
pub const COLLISION_TILE_SIZE: f32 = 1.0 / COLLISION_TILES_PER_MAP_TILE as f32;

/// Unit distance to waypoint to consider it visited.
pub const DISTANCE_TO_WAYPOINT_EPS: f32 = 0.1;

pub const MAX_ATTACK_DURATION: f32 = 180.0;

/// attack_optimizer
pub const MAX_UNIT_DROP_TIME: f32 = 20.0;

pub const SHAPE_SERIALIZE_ROUND_DIGITS: u32 = 2;

pub const DROP_ZONE_TILE_EVEN_COLOR: ShapeColor = ShapeColor::new(50, 70, 40);
pub const DROP_ZONE_TILE_ODD_COLOR: ShapeColor = ShapeColor::new(30, 30, 10);
pub const DROP_ZONE_BORDER_TILE_EVEN_COLOR: ShapeColor = ShapeColor::new(50, 70, 0);
pub const DROP_ZONE_BORDER_TILE_ODD_COLOR: ShapeColor = ShapeColor::new(30, 30, 0);

pub const TILE_EVEN_COLOR: ShapeColor = ShapeColor::new(40, 40, 40);
pub const TILE_ODD_COLOR: ShapeColor = ShapeColor::new(10, 10, 10);
pub const BORDER_TILE_EVEN_COLOR: ShapeColor = ShapeColor::new(30, 60, 0);
pub const BORDER_TILE_ODD_COLOR: ShapeColor = ShapeColor::new(20, 30, 0);

pub const BUILDING_TILE_EVEN_COLOR: ShapeColor = ShapeColor::new(120, 88, 0);
pub const BUILDING_TILE_ODD_COLOR: ShapeColor = ShapeColor::new(64, 47, 0);

pub const COLLISION_TILE_COLOR: ShapeColor = ShapeColor::new(255, 0, 0);
