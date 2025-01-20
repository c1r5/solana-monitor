use std::env;

use anyhow::Ok;
use monitor::Monitor;
use server::MonitorServer;

mod constants;
mod dex;
mod monitor;
mod pumpfun;
mod server;
mod tx;

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
        }

        Ok(())
    });

    let server = tokio::spawn(async move {
        let _ = server.init(server_rx).await;
    });

    let _ = tokio::join!(subscription, receiver, server);

    Ok(())
}

#[cfg(test)]
mod test {
    

    use crate::dex::{Dex, FromLogs, GenerateData};

    #[test]
    fn decode_raylog() {
        let logs = vec![
            "Program log: ray_log: A+0tMAMgAAAAKWGzAgAAAAABAAAAAAAAAO0tMAMgAAAAmbRGwxIAAABuYLyWuL0AAJLrJwMAAAAA"
        ];

        let dex = Dex::from_logs(&logs);
        let data = dex.data(&logs);
    }
}
