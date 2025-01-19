use std::env;

use anyhow::Ok;
use monitor::Monitor;
use server::MonitorServer;

mod constants;
mod logs_parser;
mod models;
mod monitor;
mod pumpfun;
mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let monitor = Monitor::new(&env::var("WEBSOCKET_URL")?);
    let server = MonitorServer::new(&env::var("MONITOR_SERVER_URL")?).await?;
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    let (server_tx, server_rx) = tokio::sync::mpsc::unbounded_channel();

    let subscription = tokio::spawn(async move {
        let _ = monitor.subscribe(tx).await;
    });

    let receiver = tokio::spawn(async move {
        loop {
            let Some(trade) = rx.recv().await else {
                // eprintln!("No trade in channel");
                break;
            };

            match trade.dex {
                models::Dex::PUMPFUN => {
                    server_tx.send(trade)?;
                }
                models::Dex::RAYDIUM => {}
                models::Dex::UNKNOWN => {}
            }
        }

        Ok(())
    });

    let server = tokio::spawn(async move {
        let _ = server.init(server_rx).await;
    });

    let _ = tokio::join!(subscription, receiver, server);

    Ok(())
}
