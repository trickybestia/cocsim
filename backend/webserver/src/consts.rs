pub const UNIX_SOCKET_PATH: &str = "/var/run/cocsim-webserver.sock";

pub const FPS: usize = 60;
pub const SHOWCASE_MAP: &str = "single_player/no_flight_zone";

pub const OPTIMIZE_ATTACK_STEPS: usize = 20;
pub const OPTIMIZE_ATTACK_ITERATIONS: usize = 2000;
pub const OPTIMIZE_ATTACK_ITERATIONS_PER_STEP: usize =
    OPTIMIZE_ATTACK_ITERATIONS / OPTIMIZE_ATTACK_STEPS;
