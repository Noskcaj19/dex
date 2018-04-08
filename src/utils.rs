use serenity::{self, model::id::*, CACHE};

pub fn get_user_color(guild_id: &GuildId, user_id: &UserId) -> Option<serenity::utils::Colour> {
    use serenity::{model::id::*, CACHE};
    let cache = CACHE.read();

    let guild = cache.guilds.get(guild_id)?.read();
    let member = guild.members.get(user_id)?;

    let primary_role = member.roles.get(0)?;
    Some(guild.roles.get(primary_role).unwrap().colour)
}
