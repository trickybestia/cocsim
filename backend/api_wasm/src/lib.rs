use bytes::Bytes;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    Blob,
    BlobPropertyBag,
    js_sys::{
        Array,
        JSON,
        Uint8Array,
    },
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(msg: String);
}

#[wasm_bindgen(start)]
fn start() {
    console_error_panic_hook::set_once();
}

async fn blob_to_bytes(blob: &Blob) -> Bytes {
    let array_buffer = JsFuture::from(blob.array_buffer()).await.unwrap();

    Uint8Array::new(&array_buffer).to_vec().into()
}

fn blob_from_slice(data: &[u8], r#type: &str) -> Blob {
    let options = BlobPropertyBag::new();
    options.set_type(r#type);

    let blob_parts = Array::new();
    blob_parts.push(&Uint8Array::from(data));

    Blob::new_with_u8_array_sequence_and_options(&blob_parts, &options).unwrap()
}

#[wasm_bindgen]
pub async fn compose_base_images(left: Vec<Blob>, right: Vec<Blob>) -> Result<Blob, String> {
    let mut left_images_bytes = Vec::new();

    for blob in left {
        left_images_bytes.push(blob_to_bytes(&blob).await);
    }

    let mut right_images_bytes = Vec::new();

    for blob in right {
        right_images_bytes.push(blob_to_bytes(&blob).await);
    }

    match api_base::compose_base_images(left_images_bytes, right_images_bytes) {
        Ok(result) => Ok(blob_from_slice(result.as_slice(), "image/jpeg")),
        Err(error) => Err(error.to_string()),
    }
}

#[wasm_bindgen]
pub async fn get_game_types() -> JsValue {
    JSON::parse(&serde_json::to_string(&api_base::get_game_types()).unwrap()).unwrap()
}

#[wasm_bindgen]
pub async fn get_showcase_attack_base_image() -> Blob {
    blob_from_slice(
        api_base::get_showcase_attack_base_image().as_slice(),
        "image/jpeg",
    )
}

#[wasm_bindgen]
pub async fn get_showcase_attack() -> JsValue {
    JSON::parse(&serde_json::to_string(&api_base::get_showcase_attack()).unwrap()).unwrap()
}

#[wasm_bindgen]
pub async fn reverse_projection(image: Blob) -> Result<Blob, String> {
    let image_bytes = blob_to_bytes(&image).await;

    match api_base::reverse_projection(image_bytes) {
        Ok(result) => Ok(blob_from_slice(result.as_slice(), "image/jpeg")),
        Err(error) => Err(error.to_string()),
    }
}
