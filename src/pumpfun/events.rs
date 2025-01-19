use anchor_client::solana_sdk::pubkey::Pubkey;
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct CreateEvent {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub user: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct TradeEvent {
    pub mint: Pubkey,
    pub sol_amount: u64,
    pub token_amount: u64,
    pub is_buy: bool,
    pub user: Pubkey,
    pub timestamp: i64,
    pub virtual_sol_reserves: u64,
    pub virtual_token_reserves: u64,
    pub real_sol_reserves: u64,
    pub real_token_reserves: u64,
}

#[derive(Debug)]
pub enum Event {
    Create(CreateEvent),
    Trade(TradeEvent),
    UNKNOWN
}

impl Event {
    pub fn parse_event(data: &[u8]) -> Event {
        let sliced = &data[8..];

        if let Ok(create_event) = CreateEvent::try_from_slice(sliced) {
            return Event::Create(create_event);
        }

        if let Ok(trade_event) = TradeEvent::try_from_slice(sliced) {
            return Event::Trade(trade_event);
        }

        Event::UNKNOWN
    }
}
