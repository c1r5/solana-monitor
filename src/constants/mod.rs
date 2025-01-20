/// Constants related to program accounts and authorities
pub mod accounts {
    use anchor_client::solana_sdk::{pubkey, pubkey::Pubkey};
    /// Public key for the Pump.fun program
    pub const PUMPFUN: Pubkey = pubkey!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P");
    /// Public key for the raydium concentraed liquidity
    pub const RAYDIUM_CAMM: Pubkey = pubkey!("CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK");
    /// Public key for the raydium concentraed liquidity
    pub const RAYDIUM_AMM: Pubkey = pubkey!("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8");
}
