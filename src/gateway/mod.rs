// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use futures::{SinkExt, StreamExt};
use log::{debug, info};
use poem::listener::TcpListener;
use poem::web::websocket::{Message, WebSocket};
use poem::web::Data;
use poem::{get, handler, EndpointExt, IntoResponse, Route, Server};

use crate::errors::Error;

#[handler]
fn ws(ws: WebSocket, sender: Data<&tokio::sync::broadcast::Sender<Message>>) -> impl IntoResponse {
    let sender = sender.clone();
    let mut receiver = sender.subscribe();
    ws.on_upgrade(move |socket| async move {
        let (mut sink, mut stream) = socket.split();

        tokio::spawn(async move {
            while let Some(Ok(msg)) = stream.next().await {
                if let Message::Text(text) = msg {
                    if sender.send(Message::text(text)).is_err() {
                        break;
                    }
                }
            }
        });

        tokio::spawn(async move {
            while let Ok(msg) = receiver.recv().await {
                if sink.send(msg).await.is_err() {
                    break;
                }
            }
        });
    })
}

pub async fn start_gateway() -> Result<(), Error> {
    info!(target: "symfonia::gateway", "Starting gateway server");
    let ws_app = Route::new().at(
        "/",
        get(ws.data(tokio::sync::mpsc::channel::<Message>(32).0)),
    );
    let bind = std::env::var("API_BIND").unwrap_or_else(|_| String::from("localhost:3001"));
    debug!(target: "symfonia::gateway", "Binding to {}", bind);
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(ws_app)
        .await?;
    debug!(target: "symfonia::gateway", "Gateway server started");
    Ok(())
}
