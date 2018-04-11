use failure::Error;
use serenity::model::id::{ChannelId, GuildId};
use toml;

use std::env;
use std::fs::OpenOptions;
use std::io::Read;

use errors::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Preferences {
    token: String,
    timestamp_fmt: Option<String>,
    guild: Option<GuildId>,
    channel: Option<ChannelId>,
}

impl Preferences {
    pub fn load() -> Result<Preferences, Error> {
        let home_dir = env::home_dir().ok_or(HomeDirError)?;
        let mut file = OpenOptions::new()
            .read(true)
            .open(home_dir.join(".config/ded/config.toml"))?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;

        let preferences: Preferences = toml::from_str(&buf)?;
        Ok(preferences)
    }

    pub fn token(&self) -> String {
        self.token.clone()
    }

    pub fn timestamp_fmt(&self) -> String {
        self.timestamp_fmt
            .clone()
            .unwrap_or_else(|| "%-I:%-M".to_owned())
    }

    pub fn previous_guild(&self) -> Option<GuildId> {
        self.guild
    }
    pub fn previous_channel(&self) -> Option<ChannelId> {
        self.channel
    }
}
