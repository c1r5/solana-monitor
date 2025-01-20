use anchor_client::solana_sdk::pubkey::Pubkey;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::dex::Dex;

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TxType {
    CREATE,
    SWAP,
    #[default]
    UNKNOWN,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tx {
    pub signature: String,
    pub dex: Dex,
    pub trader: Pubkey,
    pub mint: Pubkey,
    pub is_buy: bool,
    pub tx_type: TxType,
    pub sol_amount: Option<u64>,
    pub token_amount: Option<u64>,
}

impl Default for Tx {
    fn default() -> Self {
        Self {
            signature: Default::default(),
            dex: Dex::default(),
            trader: Pubkey::default(),
            mint: Pubkey::default(),
            is_buy: Default::default(),
            tx_type: TxType::default(),
            sol_amount: Default::default(),
            token_amount: Default::default(),
        }
    }
}

impl ToString for Tx {
    fn to_string(&self) -> String {
        let trade = json!({
            "signature":self.signature,
            "dex": &self.dex,
            "trader":self.trader.to_string(),
            "mint":self.mint.to_string(),
            "is_buy":self.is_buy,
            "tx_type": &self.tx_type,
            "sol_amount":self.sol_amount,
            "token_amount": self.token_amount
        });
        return serde_json::to_string(&trade).expect("error in string converter");
    }
}
