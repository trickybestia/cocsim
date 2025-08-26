use cocsim::test_maps::load_test_map_raw;

use crate::consts::SHOWCASE_MAP;

pub fn get_showcase_attack_base_image() -> Vec<u8> {
    let (_, base_image) =
        load_test_map_raw(SHOWCASE_MAP).expect("Map should be loaded successfully");

    base_image
}
