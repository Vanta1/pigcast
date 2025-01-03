use thiserror::Error;

#[derive(Error, Debug)]
pub enum PigError {
    #[error("failed to open config file")]
    LoadConfig(#[from] std::io::Error),
    #[error("failed to parse config file")]
    ParseConfig(#[from] toml::de::Error),
}
