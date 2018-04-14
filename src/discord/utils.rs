use serenity::model::channel::{Channel, Message};
use serenity::model::event::MessageUpdateEvent;
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

pub fn update_msg(msg: &mut Message, update: MessageUpdateEvent) {
    if let Some(kind) = update.kind {
        msg.kind = kind;
    }
    if let Some(content) = update.content {
        msg.content = content;
    }
    // if let Some(nonce) = update.nonce {
    //     msg.nonce = nonce;
    // }
    if let Some(tts) = update.tts {
        msg.tts = tts;
    }
    if let Some(pinned) = update.pinned {
        msg.pinned = pinned;
    }
    if let Some(timestamp) = update.timestamp {
        msg.timestamp = timestamp;
    }
    if let Some(edited_timestamp) = update.edited_timestamp {
        msg.edited_timestamp = Some(edited_timestamp);
    }
    if let Some(author) = update.author {
        msg.author = author;
    }
    if let Some(mention_everyone) = update.mention_everyone {
        msg.mention_everyone = mention_everyone;
    }
    if let Some(mentions) = update.mentions {
        msg.mentions = mentions;
    }
    if let Some(mention_roles) = update.mention_roles {
        msg.mention_roles = mention_roles;
    }
    if let Some(attachments) = update.attachments {
        msg.attachments = attachments;
    }
    // if let Some(embeds) = update.embeds {
    //     msg.embeds = embeds;
    // }
}
