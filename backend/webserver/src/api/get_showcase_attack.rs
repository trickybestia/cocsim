use axum::Json;
use serde_json::Value;
use tokio::task::spawn_blocking;

pub async fn get_showcase_attack() -> Json<Value> {
    spawn_blocking(api_base::get_showcase_attack)
        .await
        .expect("Should not panic")
        .into()
}
