use axum::Json;
use cocsim::BuildingType;
use serde_json::{
    Value,
    json,
};

pub async fn get_building_types() -> Json<Vec<Value>> {
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
