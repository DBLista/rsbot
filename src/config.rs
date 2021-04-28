use regex::Regex;
use serde::{Deserialize, Serialize};
use serenity::async_trait;
use serenity::client::Context;
use serenity::prelude::TypeMapKey;
use tokio::sync::RwLock;

use std::sync::Arc;
use std::{fs, io};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Config {
    pub token: String,
    pub guild: u64,
    pub role_2137: u64,
    pub role_2137_active: u64,
    pub role_muted: u64,
    pub time_h: u32,
    pub time_m: u32,
    pub every_secs: u64,

    trigger: String,

    #[serde(skip)]
    trigger_regex: Option<Regex>,
}

impl Config {
    /// Load config either from specified path, or in case of an error - from env variables
    pub fn load(path: &str) -> io::Result<Config> {
        let cfg = Self::load_config_file(path).unwrap_or_else(|_| {
            info!("failed to read config file, switching to env");
            envy::from_env().unwrap()
        });
        Ok(cfg)
    }

    fn load_config_file(path: &str) -> io::Result<Config> {
        let contents = fs::read_to_string(path)?;
        let cfg: Config = toml::from_str(&contents)?;
        Ok(cfg)
    }

    pub fn trigger<'a>(&mut self) -> &Regex {
        if self.trigger_regex.is_none() {
            let re = Regex::new(&self.trigger).unwrap();
            self.trigger_regex = Some(re)
        }

        self.trigger_regex.as_ref().unwrap()
    }
}

pub struct ConfigContainer;

impl TypeMapKey for ConfigContainer {
    type Value = Arc<RwLock<Config>>;
}

#[async_trait]
pub trait GetConfig {
    async fn config_lock(&self) -> Arc<RwLock<Config>>;
}

#[async_trait]
impl GetConfig for Context {
    async fn config_lock(&self) -> Arc<RwLock<Config>> {
        let data_read = self.data.read().await;
        data_read
            .get::<ConfigContainer>()
            .expect("Expected ConfigContainer in TypeMap.")
            .clone()
    }
}
