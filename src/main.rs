mod config;
mod events;
mod interval;
mod utils;

use std::sync::Arc;

use crate::config::*;

use serenity::client::bridge::gateway::GatewayIntents;
use serenity::client::Client;

use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let cfg = Config::load("config.toml").unwrap_or_else(|e| {
        println!("failed to read config file, switching to env");
        envy::from_env().unwrap()
    });

    let cfg_lock = Arc::new(RwLock::new(cfg.clone()));

    let mut client = Client::builder(cfg.token)
        .event_handler(events::Handler)
        .intents(GatewayIntents::GUILDS | GatewayIntents::GUILD_MEMBERS)
        .await
        .expect("error creating client");

    {
        let mut lock = client.data.write().await;
        lock.insert::<config::Container>(cfg_lock);
    }

    if let Err(why) = client.start().await {
        eprintln!("error while running the client: {:?}", why);
    }
}
