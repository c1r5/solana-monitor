use arrform::{arrform, ArrForm};
use serde::{Deserialize, Serialize};
use solana_program::{
    msg,
    // entrypoint::ProgramResult,
    pubkey::Pubkey,
};

pub const LOG_SIZE: usize = 256;

#[macro_export]
macro_rules! check_assert_eq {
    ($input:expr, $expected:expr, $msg:expr, $err:expr) => {
        if $input != $expected {
            log_keys_mismatch(concat!($msg, " mismatch:"), $input, $expected);
            return Err($err.into());
        }
    };
}

pub fn log_keys_mismatch(msg: &str, input: Pubkey, expected: Pubkey) {
    msg!(arrform!(
        LOG_SIZE,
        "ray_log: {} input:{}, expected:{}",
        msg,
        input,
        expected
    )
    .as_str());
}

/// LogType enum
#[derive(Debug)]
pub enum LogType {
    Init(InitLog),
    Deposit(DepositLog),
    Withdraw(WithdrawLog),
    SwapBaseIn(SwapBaseInLog),
    SwapBaseOut(SwapBaseOutLog),
}

impl LogType {
    pub fn from_u8(bytes: Vec<u8>) -> Self {
        match bytes[0] {
            0 => LogType::Init(bincode::deserialize::<InitLog>(&bytes).unwrap()),
            1 => LogType::Deposit(bincode::deserialize::<DepositLog>(&bytes).unwrap()),
            2 => LogType::Withdraw(bincode::deserialize::<WithdrawLog>(&bytes).unwrap()),
            3 => LogType::SwapBaseIn(bincode::deserialize::<SwapBaseInLog>(&bytes).unwrap()),
            4 => LogType::SwapBaseOut(bincode::deserialize::<SwapBaseOutLog>(&bytes).unwrap()),
            _ => unreachable!(),
        }
    }

    pub fn into_u8(&self) -> u8 {
        match self {
            LogType::Init(_) => 0u8,
            LogType::Deposit(_) => 1u8,
            LogType::Withdraw(_) => 2u8,
            LogType::SwapBaseIn(_) => 3u8,
            LogType::SwapBaseOut(_) => 4u8,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct InitLog {
    pub log_type: u8,
    pub time: u64,
    pub pc_decimals: u8,
    pub coin_decimals: u8,
    pub pc_lot_size: u64,
    pub coin_lot_size: u64,
    pub pc_amount: u64,
    pub coin_amount: u64,
    pub market: Pubkey,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DepositLog {
    pub log_type: u8,
    // input
    pub max_coin: u64,
    pub max_pc: u64,
    pub base: u64,
    // pool info
    pub pool_coin: u64,
    pub pool_pc: u64,
    pub pool_lp: u64,
    pub calc_pnl_x: u128,
    pub calc_pnl_y: u128,
    // calc result
    pub deduct_coin: u64,
    pub deduct_pc: u64,
    pub mint_lp: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WithdrawLog {
    pub log_type: u8,
    // input
    pub withdraw_lp: u64,
    // user info
    pub user_lp: u64,
    // pool info
    pub pool_coin: u64,
    pub pool_pc: u64,
    pub pool_lp: u64,
    pub calc_pnl_x: u128,
    pub calc_pnl_y: u128,
    // calc result
    pub out_coin: u64,
    pub out_pc: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SwapBaseInLog {
    pub log_type: u8,
    // input
    pub amount_in: u64,
    pub minimum_out: u64,
    pub direction: u64,
    // user info
    pub user_source: u64,
    // pool info
    pub pool_coin: u64,
    pub pool_pc: u64,
    // calc result
    pub out_amount: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SwapBaseOutLog {
    pub log_type: u8,
    // input
    pub max_in: u64,
    pub amount_out: u64,
    pub direction: u64,
    // user info
    pub user_source: u64,
    // pool info
    pub pool_coin: u64,
    pub pool_pc: u64,
    // calc result
    pub deduct_in: u64,
}

pub fn encode_ray_log<T: Serialize>(log: T) {
    // encode
    let bytes = bincode::serialize(&log).unwrap();
    let mut out_buf = Vec::new();
    out_buf.resize(bytes.len() * 4 / 3 + 4, 0);
    let bytes_written = base64::encode_config_slice(bytes, base64::STANDARD, &mut out_buf);
    out_buf.resize(bytes_written, 0);
    let msg_str = unsafe { std::str::from_utf8_unchecked(&out_buf) };
    msg!(arrform!(LOG_SIZE, "ray_log: {}", msg_str).as_str());
}

pub fn decode_ray_log(log: &str) -> LogType {
    let bytes = base64::decode_config(log, base64::STANDARD).unwrap();
    LogType::from_u8(bytes)
}
