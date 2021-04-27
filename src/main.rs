#![feature(proc_macro_hygiene, decl_macro)]

use crate::config::*;

mod bot;
mod config;
mod utils;
mod web;

#[macro_use]
extern crate rocket;

#[tokio::main]
async fn main() {
    let cfg = Config::load("config.toml").unwrap_or_else(|_| {
        info!("failed to read config file, switching to env");
        envy::from_env().unwrap()
    });

    let mut c = bot::new(&cfg).await.expect("failed to start client");

    let cache_http = c.cache_and_http.clone();

    tokio::spawn(async move {
        web::run(&cfg, Box::new(cache_http))
            .await
            .expect("failed to start web app");
    });

    c.start().await.expect("failed to start bot");
}
