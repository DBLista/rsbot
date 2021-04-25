use std::sync::Arc;
use std::time::Duration;

use chrono::{Timelike, Utc};
use chrono_tz::Europe;
use serenity::client::Context;
use serenity::http::CacheHttp;
use serenity::model::id::RoleId;
use tokio::sync::RwLock;
use tokio::time::interval;

use crate::config::*;
use crate::utils::*;

async fn interval_task(cfg_lock: &Arc<RwLock<Config>>, ctx: Arc<Context>) -> Result<(), Error> {
    let cfg = cfg_lock.read().await;

    let mut members = ctx
        .cache()
        .ok_or(Error::None("expected cache"))?
        .guild(cfg.guild)
        .await
        .ok_or(Error::None("expected guild"))?
        .members;

    let time = Utc::now().with_timezone(&Europe::Warsaw).time();
    let Time { h, m } = cfg.time;

    // 21:37, 21:36
    if time.hour() == h && (time.minute() == m || time.minute() == m - 1) {
        println!("{}:{} incoming", h, m);

        let vec: Vec<_> = members
            .iter_mut()
            .filter(|(_, m)| m.roles.contains(&RoleId(cfg.role_2137)))
            .collect();
        println!("got {} users to mute", vec.len());

        for (id, m) in vec {
            let res = m
                .add_roles(
                    &ctx,
                    &[RoleId(cfg.role_muted), RoleId(cfg.role_2137_active)],
                )
                .await;

            match res {
                Ok(_) => println!("muted {}", m.user.tag()),
                Err(why) => eprintln!("failed to mute {}: {:?}", id, why),
            }
        }
    } else {
        let vec: Vec<_> = members
            .iter_mut()
            .filter(|(_, m)| m.roles.contains(&RoleId(cfg.role_2137_active)))
            .collect();
        println!("got {} users to unmute", vec.len());

        for (id, m) in vec {
            let res = m
                .remove_roles(
                    ctx.http().as_ref(),
                    &[RoleId(cfg.role_2137_active), RoleId(cfg.role_muted)],
                )
                .await;

            match res {
                Ok(_) => println!("unmuted {}", m.user.tag()),
                Err(why) => eprintln!("failed to unmute {}: {:?}", id, why),
            }
        }
    }

    Ok(())
}

pub async fn spawn(cfg_lock: Arc<RwLock<Config>>, ctx: Arc<Context>) {
    let cfg = cfg_lock.read().await;
    let secs = cfg.every_secs;

    let c = cfg_lock.to_owned();
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(secs));

        println!("unmute thread started");

        loop {
            interval.tick().await;
            if let Err(why) = interval_task(&c, ctx.to_owned()).await {
                eprintln!("error while unmuting: {:?}", why);
            }
        }
    });
}
