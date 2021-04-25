use crate::config::*;
use crate::interval;
use crate::utils::*;
use serenity::async_trait;
use serenity::client::bridge::gateway::ChunkGuildFilter;
use serenity::client::{Context, EventHandler};
use serenity::model::channel::Message;
use serenity::model::event::GuildMembersChunkEvent;
use serenity::model::gateway::Ready;
use serenity::model::guild::Guild;
use serenity::model::id::GuildId;
use std::sync::Arc;

const GET_MEMBERS_NONCE: &str = "GET_MEMBERS";

pub struct Handler;

impl Handler {
    async fn handle_message(&self, ctx: Context, msg: Message) -> Result<(), Error> {
        let lock = ctx.config_lock().await;
        let mut cfg = lock.write().await;

        if msg.guild_id != Some(GuildId(cfg.guild)) {
            return Ok(());
        }

        if cfg.trigger().is_match(&msg.content) {
            println!("adding 2137 role to {}", msg.author.tag());

            let mut member = msg.member(&ctx).await?;
            member.add_role(&ctx, cfg.role_2137).await?;
        }

        Ok(())
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn guild_create(&self, ctx: Context, g: Guild, _: bool) {
        let lock = ctx.config_lock().await;
        let cfg = lock.read().await;

        if g.id == cfg.guild {
            println!("got guild {}", g.name);
            println!("requesting chunk with nonce '{}'", GET_MEMBERS_NONCE);

            ctx.shard.chunk_guild(
                g.id,
                None,
                ChunkGuildFilter::None,
                Some(GET_MEMBERS_NONCE.into()),
            );
        }
    }

    async fn guild_members_chunk(&self, ctx: Context, chunk: GuildMembersChunkEvent) {
        if chunk.nonce == Some(GET_MEMBERS_NONCE.to_string()) {
            println!(
                "got chunk with nonce '{}' of size {}",
                GET_MEMBERS_NONCE,
                chunk.members.len()
            );

            let cfg_lock = ctx.config_lock().await;
            interval::spawn(cfg_lock, Arc::new(ctx)).await;
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if let Err(why) = self.handle_message(ctx, msg).await {
            eprintln!("failed to handle message: {:?}", why);
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("ready as {}", ready.user.tag());
    }
}
