use api_base::SendRecvError;
use axum::{
    extract::{
        WebSocketUpgrade,
        ws::{
            Message,
            WebSocket,
        },
    },
    response::Response,
};
use log::warn;
use tokio::{
    select,
    task::spawn_blocking,
};

pub async fn optimize_attack(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(async |socket| {
        if let Err(err) = optimize_attack_internal(socket).await {
            warn!("optimize_attack_internal finished with error: {err:#?}");
        }
    })
}

async fn optimize_attack_internal(mut socket: WebSocket) -> anyhow::Result<()> {
    let (send_tx, mut send_rx) = tokio::sync::mpsc::channel::<String>(10);
    let (recv_tx, mut recv_rx) = tokio::sync::mpsc::channel::<String>(10);

    let send = move |s: String| match send_tx.blocking_send(s) {
        Ok(_) => Ok(()),
        Err(_) => Err(SendRecvError::Cancel),
    };
    let recv = move || match recv_rx.blocking_recv() {
        Some(s) => Ok(s),
        None => Err(SendRecvError::Cancel),
    };

    let mut join_handle = spawn_blocking(|| api_base::optimize_attack(send, recv));
    let mut join_handle_awaited = false;

    loop {
        select! {
            result = &mut join_handle => {
                result??;
                join_handle_awaited = true;

                break;
            },
            value_to_send = send_rx.recv() => {
                match value_to_send {
                    None => break,
                    Some(value_to_send) => {
                        socket.send(Message::Text(value_to_send.into())).await?;
                    }
                }
            }
            recv_value = socket.recv() => {
                match recv_value {
                    None => break,
                    Some(recv_value) => {
                        recv_tx.send(recv_value?.into_text()?.as_str().to_owned()).await?;
                    }
                }
            }
        }
    }

    // dropping these causes send and recv to return Err(SendRecvError::Cancel) in
    // api_base::optimize_attack
    drop(send_rx);
    drop(recv_tx);

    if !join_handle_awaited {
        join_handle.await??;
    }

    socket.send(Message::Close(None)).await?;

    Ok(())
}
