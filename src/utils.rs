use serenity::model::channel::Channel;
use serenity::model::channel::Message;
use serenity::model::guild::{Guild, Member};
use serenity::prelude::RwLock;
use serenity::{self, model::id::*, CACHE};

use std::sync::Arc;

pub fn get_user_color(guild_id: &GuildId, user_id: &UserId) -> Option<serenity::utils::Colour> {
    use serenity::{model::id::*, CACHE};
    let cache = CACHE.read();

    let guild = cache.guilds.get(guild_id)?.read();
    let member = guild.members.get(user_id)?;

    let primary_role = member.roles.get(0)?;
    Some(guild.roles.get(primary_role).unwrap().colour)
}

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
