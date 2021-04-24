use serde::{Deserialize, Serialize};
use serenity::async_trait;
use serenity::client::Context;
use serenity::prelude::TypeMapKey;
use std::sync::Arc;
use std::{fs, io};
use tokio::sync::RwLock;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Time {
    pub h: u32,
    pub m: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Config {
    pub token: String,
    pub guild: u64,
    pub role_2137: u64,
    pub role_2137_active: u64,
    pub role_muted: u64,
    pub time: Time,
    pub every_secs: u64,
}

impl Config {
    pub fn load(path: &str) -> io::Result<Config> {
        let contents = fs::read_to_string(path)?;
        let cfg: Config = toml::from_str(&contents)?;
        Ok(cfg)
    }
}

pub struct Container;

impl TypeMapKey for Container {
    type Value = Arc<RwLock<Config>>;
}

#[async_trait]
pub trait GetConfig {
    async fn get_config(self) -> Arc<RwLock<Config>>;
}

#[async_trait]
impl GetConfig for &Context {
    async fn get_config(self) -> Arc<RwLock<Config>> {
        let data_read = self.data.read().await;
        data_read
            .get::<Container>()
            .expect("Expected ConfigContainer in TypeMap.")
            .clone()
    }
}
