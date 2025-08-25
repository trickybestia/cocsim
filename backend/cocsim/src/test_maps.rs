use std::{
    env,
    fs::File,
    io::{
        BufReader,
        Read,
    },
    path::PathBuf,
};

use include_dir::{
    Dir,
    include_dir,
};
use zip::ZipArchive;

use crate::{
    Map,
    ValidatedMap,
};

static TEST_MAPS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/../../test_maps");

pub fn load_test_map_raw(name: &str) -> anyhow::Result<(String, Vec<u8>)> {
    let path = PathBuf::from(env::var("TEST_MAPS_PATH")?)
        .join(name)
        .with_extension("zip");
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut archive = ZipArchive::new(reader)?;

    let mut map_image = Vec::new();
    let mut map_json = String::new();

    archive.by_name("map.json")?.read_to_string(&mut map_json)?;
    archive.by_name("map.jpg")?.read_to_end(&mut map_image)?;

    Ok((map_json, map_image))
}

pub fn load_test_map(name: &str) -> anyhow::Result<(ValidatedMap, Vec<u8>)> {
    let (map_json, map_image) = load_test_map_raw(name)?;

    let map = serde_json::from_str::<Map>(&map_json)?;
    let validated_map = ValidatedMap::try_from(map)?;

    Ok((validated_map, map_image))
}
