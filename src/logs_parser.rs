use crate::models::Dex;

const PUMPFUN_DISCRIMINATOR: &str = "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P";
const RAYDIUM_DISCRIMINATOR: &str = "ray_log";


pub struct LogsParser {
    logs: Vec<String>,
}

impl LogsParser {
    pub fn new<I, T>(logs: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: AsRef<str>,
    {
        LogsParser {
            logs: logs.into_iter().map(|s| s.as_ref().to_string()).collect(),
        }
    }

    pub fn data(&self) -> Vec<String> {
        self.logs
            .iter()
            .filter(|log| log.starts_with("Program data:"))
            .flat_map(|log| log.split("Program data:").map(|s| s.trim().to_string()))
            .filter(|log| !log.is_empty())
            .collect::<Vec<String>>()
        // .collect::<Option<String>>()
    }

    pub fn dex(&self) -> Dex {
        if self
            .logs
            .iter()
            .any(|log| log.contains(PUMPFUN_DISCRIMINATOR))
        {
            Dex::PUMPFUN
        } else if self
            .logs
            .iter()
            .any(|log| log.contains(RAYDIUM_DISCRIMINATOR))
        {
            Dex::RAYDIUM
        } else {
            Dex::default()
        }
    }
}
