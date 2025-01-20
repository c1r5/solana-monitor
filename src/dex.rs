use log::info;
use serde::{Deserialize, Serialize};

const PUMPFUN_DISCRIMINATOR: &str = "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P";
const RAYDIUMV4_DISCRIMINATOR: &str = "ray_log:";

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum Dex {
    Pumpfun,
    RaydiumAMM,
    #[default]
    Unknown,
}

pub trait FromLogs {
    fn from_logs<I, T>(logs: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: AsRef<str>;
}

impl FromLogs for Dex {
    fn from_logs<I, T>(logs: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: AsRef<str>,
    {
        let logs = logs
            .into_iter()
            .map(|s| s.as_ref().to_string())
            .collect::<Vec<String>>();

        if logs.iter().any(|log| log.contains(PUMPFUN_DISCRIMINATOR)) {
            Dex::Pumpfun
        } else if logs.iter().any(|log| log.contains(RAYDIUMV4_DISCRIMINATOR)) {
            Dex::RaydiumAMM
        } else {
            Dex::default()
        }
    }
}

pub trait GenerateData {
    fn data<I, T>(&self, logs: I) -> Vec<String>
    where
        I: IntoIterator<Item = T>,
        T: AsRef<str>;
}

impl GenerateData for Dex {
    fn data<I, T>(&self, logs: I) -> Vec<String>
    where
        I: IntoIterator<Item = T>,
        T: AsRef<str>,
    {
        let delimiter = match self {
            Dex::Pumpfun | Dex::Unknown => "Program data:",
            Dex::RaydiumAMM => "ray_log:",
        };

        logs.into_iter()
            .map(|s| s.as_ref().to_string())
            .filter(|s| s.contains(&delimiter))
            .flat_map(|s| {
                s.split(&delimiter)
                    .map(|i| i.trim().to_string())
                    .filter(|i| i != "Program log:" && !i.is_empty())
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<String>>()
    }
}
