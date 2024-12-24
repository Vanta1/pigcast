use std::fs::read_to_string;

use serde::Deserialize;
use toml::Table;

#[derive(Clone, Deserialize)]
pub struct Config {
    data: Table,
}

impl Config {
    pub fn from_file(path: String) -> Self {
        let unparsed = read_to_string(path).expect("failed to open config file");
        let parsed: Table = unparsed.parse().unwrap();

        Self { data: parsed }
    }

    pub fn data(&self) -> &Table {
        &self.data
    }
}
