use axum::Json;
use cocsim::UnitType;
use serde_json::{
    Value,
    json,
};

pub async fn get_unit_types() -> Json<Vec<Value>> {
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
