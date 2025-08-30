use std::cell::RefCell;

use api_base::SendRecvError;
use bytes::Bytes;
use log::{
    Level,
    info,
};
use tokio::select;
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
pub struct OptimizeAttackApiStream(RefCell<OptimizeAttackApiStreamState>);

enum OptimizeAttackApiStreamState {
    Created {
        recv_tx: tokio::sync::mpsc::UnboundedSender<String>,
        recv_rx: tokio::sync::mpsc::UnboundedReceiver<String>,
    },
    Working {
        recv_tx: tokio::sync::mpsc::UnboundedSender<String>,
        close_tx: Option<tokio::sync::oneshot::Sender<()>>,
    },
    Stopped,
}

#[wasm_bindgen]
impl OptimizeAttackApiStream {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        info!("OptimizeAttackApiStream::new()");

        let (recv_tx, recv_rx) = tokio::sync::mpsc::unbounded_channel();

        Self(RefCell::new(OptimizeAttackApiStreamState::Created {
            recv_tx,
            recv_rx,
        }))
    }

    #[wasm_bindgen]
    pub fn send(&self, data: String) {
        info!("OptimizeAttackApiStream::send()");

        match &*self.0.borrow() {
            OptimizeAttackApiStreamState::Created {
                recv_tx,
                recv_rx: _,
            } => recv_tx.send(data).unwrap(),
            OptimizeAttackApiStreamState::Working {
                recv_tx,
                close_tx: _,
            } => recv_tx.send(data).unwrap(),
            OptimizeAttackApiStreamState::Stopped => panic!(),
        }
    }

    #[wasm_bindgen]
    pub async fn run(&self, on_message: Function) {
        info!("OptimizeAttackApiStream::run()");

        let state = self.0.replace(OptimizeAttackApiStreamState::Stopped);

        let (mut recv_rx, mut close_rx) =
            if let OptimizeAttackApiStreamState::Created { recv_tx, recv_rx } = state {
                let (close_tx, close_rx) = tokio::sync::oneshot::channel();

                self.0.replace(OptimizeAttackApiStreamState::Working {
                    recv_tx,
                    close_tx: Some(close_tx),
                });

                (recv_rx, close_rx)
            } else {
                self.0.replace(state);

                return;
            };

        let (send_tx, mut send_rx) = tokio::sync::mpsc::unbounded_channel();

        let send = move |s: String| match send_tx.send(s) {
            Ok(_) => Ok(()),
            Err(_) => Err(SendRecvError::Cancel),
        };

        let recv = move || match recv_rx.blocking_recv() {
            Some(s) => Ok(s),
            None => Err(SendRecvError::Cancel),
        };

        rayon::spawn(move || api_base::optimize_attack(send, recv).unwrap());

        loop {
            select! {
                _ = &mut close_rx => break,
                data = send_rx.recv() => {
                    if let Some(data) = data {
                        let args = Array::new();
                        args.push(&JsString::from(data));

                        let _ = on_message.apply(&JsValue::UNDEFINED, &args);
                    } else {
                        break;
                    }
                }
            }
        }

        *self.0.borrow_mut() = OptimizeAttackApiStreamState::Stopped;
    }

    #[wasm_bindgen]
    pub fn close(&self) {
        info!("OptimizeAttackApiStream::close()");

        if let OptimizeAttackApiStreamState::Working {
            recv_tx: _,
            close_tx,
        } = &mut *self.0.borrow_mut()
        {
            if let Some(close_tx) = close_tx.take() {
                let _ = close_tx.send(());
            }
        }

        *self.0.borrow_mut() = OptimizeAttackApiStreamState::Stopped;
    }
}

#[wasm_bindgen]
pub async fn reverse_projection(image: Blob) -> Result<Blob, String> {
    let image_bytes = blob_to_bytes(&image).await;

    match api_base::reverse_projection(image_bytes) {
        Ok(result) => Ok(blob_from_slice(result.as_slice(), "image/jpeg")),
        Err(error) => Err(error.to_string()),
    }
}
