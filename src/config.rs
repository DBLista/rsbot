use regex::Regex;
use serde::{Deserialize, Serialize};
use serenity::async_trait;
use serenity::client::Context;
use serenity::prelude::TypeMapKey;
use std::sync::Arc;
use std::{fs, io};
use tokio::sync::{RwLock, RwLockWriteGuard};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Time {
    pub h: u32,
    pub m: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DefaultVec<T>(Vec<T>);
impl<T> Default for DefaultVec<T> {
    fn default() -> Self {
        DefaultVec(Vec::new())
    }
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

    trigger: String,

    #[serde(skip)]
    trigger_regex: Option<Regex>,
}

impl Config {
    pub fn load(path: &str) -> io::Result<Config> {
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

pub struct Container;

impl TypeMapKey for Container {
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
            .get::<Container>()
            .expect("Expected ConfigContainer in TypeMap.")
            .clone()
    }
}
