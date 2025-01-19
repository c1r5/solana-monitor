use anchor_client::{
    solana_client::{
        nonblocking::pubsub_client::PubsubClient,
        rpc_config::{RpcTransactionLogsConfig, RpcTransactionLogsFilter},
    },
    solana_sdk::commitment_config::CommitmentConfig,
};

use base64::{prelude::BASE64_STANDARD, Engine};
use futures::StreamExt;
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    constants::accounts::PUMPFUN,
    logs_parser::LogsParser,
    models::{self, Trade, TxType},
    pumpfun::events::Event,
};

pub struct Monitor {
    url: String,
}

impl Monitor {
    pub fn new(url: &str) -> Self {
        Self {
            url: String::from(url),
        }
    }

    pub async fn subscribe(&self, tx: UnboundedSender<Trade>) -> anyhow::Result<()> {
        let pubsub = PubsubClient::new(&self.url).await?;
        let (mut stream, _) = pubsub
            .logs_subscribe(
                RpcTransactionLogsFilter::Mentions(vec![PUMPFUN.to_string()]),
                RpcTransactionLogsConfig {
                    commitment: Some(CommitmentConfig::confirmed()),
                },
            )
            .await?;

        loop {
            let Some(response) = stream.next().await else {
                break;
            };

            let mut trade = Trade::default();

            let log_messages = response.value.logs;
            let logs_parser = LogsParser::new(&log_messages);
            let dex = logs_parser.dex();
            let data = logs_parser.data();

            if data.is_empty() {
                continue;
            }

            trade.signature = response.value.signature;
            trade.dex = dex.clone();

            match dex {
                models::Dex::PUMPFUN => {
                    let events = data
                        .iter()
                        .map(|data| {
                            let decoded = BASE64_STANDARD.decode(data).unwrap();
                            Event::parse_event(&decoded)
                        })
                        .collect::<Vec<_>>();

                    for event in events {
                        match event {
                            Event::Create(create_event) => {
                                trade.trader = create_event.user;
                                trade.mint = create_event.mint;
                                trade.tx_type = TxType::CREATE;
                            }

                            Event::Trade(trade_event) => {
                                trade.trader = trade_event.user;
                                trade.is_buy = trade_event.is_buy;
                                trade.mint = trade_event.mint;
                                trade.sol_amount = Some(trade_event.sol_amount);
                                trade.token_amount = Some(trade_event.token_amount);
                                trade.tx_type = TxType::TRADE
                            }
                            Event::UNKNOWN => {}
                        }
                    }
                }
                models::Dex::RAYDIUM => {}
                models::Dex::UNKNOWN => {}
            }

            if let Err(send_err) = tx.send(trade) {
                eprintln!("{}", send_err)
            };
        }

        Ok(())
    }
}
