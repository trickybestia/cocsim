use axum::Json;
use cocsim::SpellType;
use serde_json::{
    Value,
    json,
};

pub async fn get_spell_types() -> Json<Vec<Value>> {
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
