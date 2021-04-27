pub mod events;
mod interval;

use crate::config::*;
use serenity::client::bridge::gateway::GatewayIntents;
use serenity::Client;
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn new(cfg: &Config) -> Result<Client, serenity::Error> {
    let client = Client::builder(&cfg.token)
        .event_handler(events::Handler)
        .intents(
            GatewayIntents::GUILDS | GatewayIntents::GUILD_MEMBERS | GatewayIntents::GUILD_MESSAGES,
        )
        .await
        .expect("error creating client");

    {
        let mut lock = client.data.write().await;
        lock.insert::<ConfigContainer>(Arc::new(RwLock::new(cfg.clone())));
    }

    tracing_subscriber::fmt::init();

    Ok(client)
}
