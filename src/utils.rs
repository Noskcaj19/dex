use serenity::model::channel::{Channel, Message};
use serenity::model::guild::{Guild, Member};
use serenity::model::id::*;
use serenity::prelude::RwLock;
use serenity::CACHE;

use std::sync::Arc;

pub fn guild(message: &Message) -> Option<Arc<RwLock<Guild>>> {
    guild_id(message).and_then(|guild_id| CACHE.read().guild(guild_id))
}

pub fn guild_id(message: &Message) -> Option<GuildId> {
    match message.channel_id.get().ok() {
        Some(Channel::Guild(ch)) => Some(ch.read().guild_id),
        _ => None,
    }
}

pub fn member(message: &Message) -> Option<Member> {
    guild(message).and_then(|g| g.read().members.get(&message.author.id).cloned())
}
