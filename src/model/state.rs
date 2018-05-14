use failure::Error;
use toml;

use serenity::model::id::{ChannelId, GuildId};

use std::env;
use std::fs::OpenOptions;
use std::io::{Read, Write};

use error::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct State {
    pub guild: Option<GuildId>,
    pub channel: Option<ChannelId>,
    #[serde(default = "_true")]
    pub guild_sidebar_visible: bool,
}

fn _true() -> bool {
    true
}

impl State {
    pub fn load() -> Result<State, Error> {
        let home_dir = env::home_dir().ok_or(HomeDirError)?;
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(home_dir.join(".config/dex/persistent_state.toml"))?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;

        let state: State = toml::from_str(&buf)?;
        Ok(state)
    }

    pub fn save(&self) -> Result<(), Error> {
        let home_dir = env::home_dir().ok_or(HomeDirError)?;
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(home_dir.join(".config/dex/persistent_state.toml"))?;

        let data = toml::to_string(self)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::State;
    use toml;
    #[test]
    fn clean_state() {
        let clean_state = "";
        let state: State = toml::from_str(clean_state).unwrap();

        assert!(state.guild.is_none());
        assert!(state.channel.is_none());
        assert_eq!(state.guild_sidebar_visible, true);
    }
}
