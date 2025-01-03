use std::fs::read_to_string;

use serde::{Deserialize, Serialize};
use toml::Table;

use crate::error::PigError;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Settings {
    config_dir: String,
    cache_dir: String,
}

impl Default for Settings {
    fn default() -> Self {
        let config_dir = "~/.config/pigcast".to_string();
        let cache_dir = "~/.pigcast/cache".to_string();

        Self {
            config_dir,
            cache_dir,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Feed {
    pub title: String,
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
struct FeedInfo {
    url: String,
}

#[derive(Clone, Deserialize, PartialEq, Default)]
pub struct Config {
    settings: Settings,
    feeds: Vec<Feed>,
}

impl Config {
    pub fn from_file(path: String) -> Result<Self, PigError> {
        let data: Table = read_to_string(path)?.parse()?;

        let settings: Settings =
            toml::from_str(&data["settings"].as_table().unwrap().to_string()).expect("cant parse");

        let feeds: Vec<Feed> = data["feed"]
            .as_table()
            .unwrap()
            .iter()
            .map(|(k, v)| {
                let feed_info: FeedInfo =
                    toml::from_str(&v.as_table().unwrap().to_string()).expect("");

                Feed {
                    title: k.to_string(),
                    url: feed_info.url,
                }
            })
            .collect();

        Ok(Self { settings, feeds })
    }

    pub fn _settings(&self) -> &Settings {
        &self.settings
    }

    pub fn feeds(&self) -> &Vec<Feed> {
        &self.feeds
    }
}
