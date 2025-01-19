use std::sync::Arc;

use futures::SinkExt;
use log::info;
use tokio::{
    net::TcpListener,
    sync::{mpsc::UnboundedReceiver, Mutex},
};
use tokio_tungstenite::tungstenite::Message;

use crate::models::Trade;

pub struct MonitorServer {
    pub listener: TcpListener,
}
impl MonitorServer {
    pub async fn new(host: &str) -> Result<Self, anyhow::Error> {
        let listener = TcpListener::bind(&host).await?;
        info!("Websocket server listening on: {}", &host);
        Ok(Self { listener })
    }

    pub async fn init(&self, rx: UnboundedReceiver<Trade>) {
        let arc_rx = Arc::new(Mutex::new(rx));

        while let Ok((stream, _addr)) = self.listener.accept().await {
            let arc_rx_clone = Arc::clone(&arc_rx);

            tokio::spawn(async move {
                let mut websocket = tokio_tungstenite::accept_async(stream)
                    .await
                    .expect("Error during websocket handshake ocurred");

                let mut rx = arc_rx_clone.lock().await;

                loop {
                    let Some(trade) = rx.recv().await else {
                        break;
                    };

                    let _ = websocket.send(Message::text(trade.to_string())).await;
                }
            });
        }
    }
}
