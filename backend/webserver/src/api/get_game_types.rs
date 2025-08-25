use axum::Json;
use cocsim::{
    buildings::BuildingType,
    spells::SpellType,
    units::UnitType,
};
use serde_json::{
    Value,
    json,
};

fn building_types() -> Value {
    let mut result = Vec::new();

    for building_type in inventory::iter::<BuildingType> {
        result.push(json! ({
            "name": building_type.name,
            "width": building_type.size.x,
            "height": building_type.size.y,
            "levels": building_type.levels,
            "options": building_type.options
        }));
    }

    result.into()
}

fn unit_types() -> Value {
    let mut result = Vec::new();

    for unit_type in inventory::iter::<UnitType> {
        result.push(json! ({
            "name": unit_type.name,
            "levels": unit_type.levels,
            "housingSpace": unit_type.housing_space
        }));
    }

    result.into()
}

fn spell_types() -> Value {
    let mut result = Vec::new();

    for spell_type in inventory::iter::<SpellType> {
        result.push(json! ({
            "name": spell_type.name,
            "levels": spell_type.levels,
            "housingSpace": spell_type.housing_space
        }));
    }

    result.into()
}

pub async fn get_game_types() -> Json<Value> {
    json!({
        "buildings": building_types(),
        "units": unit_types(),
        "spells": spell_types()
    })
    .into()
}
