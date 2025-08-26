use axum::Json;
use serde_json::Value;

pub async fn get_game_types() -> Json<Value> {
    api_base::get_game_types().into()
}
