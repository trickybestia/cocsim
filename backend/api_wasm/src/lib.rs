use api_base::SendRecvError;
use bytes::Bytes;
use log::{
    Level,
    info,
};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
pub use wasm_bindgen_rayon::init_thread_pool;
use web_sys::{
    Blob,
    BlobPropertyBag,
    js_sys::{
        Array,
        Function,
        JSON,
        JsString,
        Uint8Array,
    },
};

#[wasm_bindgen(start)]
fn start() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(Level::Info).unwrap();
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
pub struct OptimizeAttackApiStream {
    recv_tx: tokio::sync::mpsc::Sender<String>,
    recv_rx: Option<tokio::sync::mpsc::Receiver<String>>,
    send_tx: Option<tokio::sync::mpsc::Sender<String>>,
    send_rx: tokio::sync::mpsc::Receiver<String>,
}

#[wasm_bindgen]
impl OptimizeAttackApiStream {
    fn new() -> Self {
        let (send_tx, send_rx) = tokio::sync::mpsc::channel::<String>(10);
        let (recv_tx, recv_rx) = tokio::sync::mpsc::channel::<String>(10);

        Self {
            recv_tx,
            recv_rx: Some(recv_rx),
            send_tx: Some(send_tx),
            send_rx,
        }
    }

    #[wasm_bindgen]
    pub fn send(&self, data: String) {
        self.recv_tx.blocking_send(data).unwrap();
    }

    #[wasm_bindgen]
    pub fn close(&self) {
        info!("Closed");
    }

    #[wasm_bindgen]
    pub async fn start(&mut self, on_message: Function) {
        let send_tx = self.send_tx.take().unwrap();
        let mut recv_rx = self.recv_rx.take().unwrap();

        let send = move |s: String| match send_tx.blocking_send(s) {
            Ok(_) => Ok(()),
            Err(_) => Err(SendRecvError::Cancel),
        };
        let recv = move || match recv_rx.blocking_recv() {
            Some(s) => Ok(s),
            None => Err(SendRecvError::Cancel),
        };

        rayon::spawn(move || api_base::optimize_attack(send, recv).unwrap());

        while let Some(data) = self.send_rx.recv().await {
            let args = Array::new();
            args.push(&JsString::from(data));

            let _ = on_message.apply(&JsValue::UNDEFINED, &args);
        }
    }
}

#[wasm_bindgen]
pub fn optimize_attack_connect() -> OptimizeAttackApiStream {
    OptimizeAttackApiStream::new()
}

#[wasm_bindgen]
pub async fn reverse_projection(image: Blob) -> Result<Blob, String> {
    let image_bytes = blob_to_bytes(&image).await;

    match api_base::reverse_projection(image_bytes) {
        Ok(result) => Ok(blob_from_slice(result.as_slice(), "image/jpeg")),
        Err(error) => Err(error.to_string()),
    }
}
