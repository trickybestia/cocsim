pub const TEST_IMAGES_PATH: &str = "test_images";
pub const TEST_MAPS_PATH: &str = "test_maps";

pub const PIXELS_PER_TILE: u32 = 20;

/// Used for pathfinding.
pub const COLLISION_TILES_PER_MAP_TILE: usize = 10;
pub const COLLISION_TILE_SIZE: f32 = 1.0 / COLLISION_TILES_PER_MAP_TILE as f32;

/// Unit distance to waypoint to consider it visited.
pub const DISTANCE_TO_WAYPOINT_EPS: f32 = 0.1;

pub const FPS: u32 = 60;

pub const MAX_ATTACK_DURATION: f32 = 180.0;

pub const SHAPE_SERIALIZE_ROUND_DIGITS: u32 = 2;

pub const DROP_ZONE_TILE_EVEN_COLOR: &str = "#324628";
pub const DROP_ZONE_TILE_ODD_COLOR: &str = "#1E1E0A";
pub const DROP_ZONE_BORDER_TILE_EVEN_COLOR: &str = "#324600";
pub const DROP_ZONE_BORDER_TILE_ODD_COLOR: &str = "#1E1E00";

pub const TILE_EVEN_COLOR: &str = "#282828";
pub const TILE_ODD_COLOR: &str = "#0A0A0A";
pub const BORDER_TILE_EVEN_COLOR: &str = "#1E3C00";
pub const BORDER_TILE_ODD_COLOR: &str = "#141E00";

pub const BUILDING_TILE_EVEN_COLOR: &str = "#785800";
pub const BUILDING_TILE_ODD_COLOR: &str = "#402F00";

pub const COLLISION_TILE_COLOR: &str = "#FF0000";
