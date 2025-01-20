use anchor_client::solana_client::rpc_client::SerializableTransaction;
use anchor_client::solana_client::rpc_config::RpcBlockSubscribeFilter;
use anchor_client::{
    solana_client::{
        nonblocking::pubsub_client::PubsubClient, rpc_config::RpcBlockSubscribeConfig,
    },
    solana_sdk::commitment_config::CommitmentConfig,
};

use futures::StreamExt;
use log::info;
use solana_transaction_status::{
    option_serializer::OptionSerializer, EncodedTransaction, TransactionDetails,
    UiTransactionEncoding,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::dex::GenerateData;
use crate::{
    dex::{Dex, FromLogs},
    tx::Tx,
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

    pub async fn subscribe(&self, tx: UnboundedSender<Tx>) -> anyhow::Result<()> {
        let pubsub = PubsubClient::new(&self.url).await?;
        let (mut stream, _) = pubsub
            .block_subscribe(
                RpcBlockSubscribeFilter::All,
                Some(RpcBlockSubscribeConfig {
                    commitment: Some(CommitmentConfig::confirmed()),
                    encoding: Some(UiTransactionEncoding::JsonParsed),
                    transaction_details: Some(TransactionDetails::Full),
                    show_rewards: Some(true),
                    max_supported_transaction_version: Some(0),
                }),
            )
            .await?;

        loop {
            let Some(response) = stream.next().await else {
                break;
            };

            let Some(block) = response.value.block else {
                continue;
            };

            let Some(txs) = block.transactions else {
                continue;
            };

            for e_tx in txs {
                let Some(meta) = e_tx.meta else {
                    continue;
                };

                let OptionSerializer::Some(logs) = meta.log_messages.to_owned() else {
                    continue;
                };

                let mut trade = Tx::default();

                match &e_tx.transaction {
                    EncodedTransaction::Json(ui_transaction) => {
                        if let Some(signature) = ui_transaction.signatures.first() {
                            trade.signature = signature.to_string()
                        }
                    }
                    EncodedTransaction::Accounts(ui_accounts_list) => {
                        if let Some(signature) = ui_accounts_list.signatures.first() {
                            trade.signature = signature.to_string()
                        }
                    }

                    _ => {
                        if let Some(v_tx) = e_tx.transaction.decode() {
                            trade.signature = v_tx.get_signature().to_string();
                        }
                    }
                }

                let dex = Dex::from_logs(&logs);
                let data = dex.data(&logs);

                if data.is_empty() {
                    continue;
                }

                if let Err(send_err) = tx.send(trade) {
                    eprintln!("{}", send_err)
                };
            }
        }

        Ok(())
    }
}
