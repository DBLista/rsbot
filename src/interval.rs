use crate::config::*;
use crate::utils::*;
use chrono::{Timelike, Utc};
use chrono_tz::Europe;
use serenity::client::Context;
use serenity::http::CacheHttp;
use serenity::model::id::RoleId;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{RwLock, RwLockReadGuard};
use tokio::time::interval;

async fn interval_task(cfg: &RwLockReadGuard<'_, Config>, ctx: Arc<Context>) -> Result<(), Error> {
    let mut members = ctx
        .cache()
        .ok_or(Error::None("expected cache"))?
        .guild(cfg.guild)
        .await
        .ok_or(Error::None("expected guild"))?
        .members;

    let time = Utc::now().with_timezone(&Europe::Warsaw).time();
    let Time { h, m } = cfg.time;

    if (time.hour(), time.minute()) == (h, m) {
        println!("{}:{}!!!", h, m);

        let vec: Vec<_> = members
            .iter_mut()
            .filter(|(_, m)| m.roles.contains(&RoleId(cfg.role_2137)))
            .collect();
        println!("got {} users to mute", vec.len());

        for (id, m) in vec {
            match m
                .add_roles(
                    &ctx,
                    &[RoleId(cfg.role_muted), RoleId(cfg.role_2137_active)],
                )
                .await
            {
                Ok(_) => println!("muted {}", m.user.tag()),
                Err(why) => println!("failed to mute {}: {:?}", id, why),
            }
        }
    } else {
        let vec: Vec<_> = members
            .iter_mut()
            .filter(|(_, m)| m.roles.contains(&RoleId(cfg.role_2137_active)))
            .collect();
        println!("got {} users to unmute", vec.len());

        for (id, m) in vec {
            match m
                .remove_roles(
                    ctx.http().as_ref(),
                    &[RoleId(cfg.role_2137_active), RoleId(cfg.role_muted)],
                )
                .await
            {
                Ok(_) => println!("unmuted {}", m.user.tag()),
                Err(why) => println!("failed to unmute {}: {:?}", id, why),
            }
        }
    }

    Ok(())
}

pub async fn spawn(cfg_lock: Arc<RwLock<Config>>, ctx: Arc<Context>) {
    tokio::spawn(async move {
        let cfg = cfg_lock.read().await;

        let mut interval = interval(Duration::from_secs(cfg.every_secs));

        println!("unmute thread started");

        loop {
            interval.tick().await;
            if let Err(why) = interval_task(&cfg, ctx.to_owned()).await {
                println!("error while unmuting: {:?}", why);
            }
        }
    });
}
