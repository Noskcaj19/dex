use std::sync::mpsc::Sender;

use serenity::model::id::{ChannelId, GuildId};
use serenity::model::user::CurrentUser;

use helpers::chars::CharSet;
use models::event::Event;
use models::{preferences::Preferences, state::State};

use failure::Error;

pub struct Context {
    /// Users OAuth token
    pub token: String,
    /// Printf-line format string for displaying timestamps
    pub timestamp_fmt: String,
    /// Whether or not to use Nerd Fonts
    pub nerd_fonts: bool,

    /// Whether or not to show the guild sidebar
    pub guild_sidebar_visible: bool,

    /// Application wide event channel
    pub event_channel: Sender<Event>,

    /// Current guild
    pub guild: Option<GuildId>,
    /// Current channel
    pub channel: Option<ChannelId>,

    /// Current user
    pub current_user: Option<CurrentUser>,

    /// Charset to use throughout the app
    pub char_set: CharSet,
}

impl Context {
    pub fn new(prefs: &Preferences, state: &State, event_channel: Sender<Event>) -> Context {
        // Prefs
        let token = prefs.token.clone();

        let timestamp_fmt = prefs
            .timestamp_fmt
            .clone()
            .unwrap_or_else(|| "%_I:%M".to_owned());

        let nerd_fonts = prefs.nerd_fonts.unwrap_or(false);

        let char_set = if nerd_fonts {
            CharSet::nerd()
        } else {
            CharSet::unicode()
        };

        // State
        let channel = state.channel;
        let guild = state.guild;

        let guild_sidebar_visible = state.guild_sidebar_visible;

        let current_user = None;

        Context {
            token,
            timestamp_fmt,
            nerd_fonts,
            guild_sidebar_visible,
            event_channel,
            channel,
            guild,
            current_user,
            char_set,
        }
    }

    pub fn save_state(&self) -> Result<(), Error> {
        self.get_state().save()
    }

    pub fn get_state(&self) -> State {
        State {
            channel: self.channel,
            guild: self.guild,
            guild_sidebar_visible: self.guild_sidebar_visible,
        }
    }
}
