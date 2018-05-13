mod guild_list;
mod indicator;
mod input;
mod messages;
mod terminal;

use std::sync::mpsc::{self, Sender, SyncSender};
use std::sync::Arc;

use models::event::Event;
use models::preferences::Preferences;
use models::state::State;

use serenity::prelude::Mutex;

use termbuf;

use failure::Error;

pub struct View {
    pub terminal: terminal::Terminal,
    _event_channel: Sender<Event>,
    event_listener_killswitch: SyncSender<()>,
    pub message_view: messages::Messages,
    pub input_view: input::Input,
    pub terminal_size: termbuf::TermSize,
    pub indicator: indicator::Indicator,
    pub guild_list: guild_list::GuildList,
    pub state: Arc<Mutex<State>>,
}

impl View {
    pub fn new(
        preferences: &Preferences,
        event_channel: Sender<Event>,
        state: Arc<Mutex<State>>,
    ) -> View {
        let terminal = terminal::Terminal::new().unwrap();
        // let terminal = termbuf::TermBuf::init().unwrap();

        let terminal_size = terminal.buf.size().expect("Unable to get size");

        let (killswitch_tx, killswitch_rx) = mpsc::sync_channel(0);
        terminal.listen(event_channel.clone(), killswitch_rx);

        let message_view = messages::Messages::new(preferences.timestamp_fmt(), false);
        let input_view = input::Input::new(event_channel.clone());
        let indicator = indicator::Indicator::new(event_channel.clone());
        let guild_list = guild_list::GuildList::new();

        View {
            terminal,
            _event_channel: event_channel,
            event_listener_killswitch: killswitch_tx,
            message_view,
            input_view,
            terminal_size,
            indicator,
            guild_list,
            state,
        }
    }

    pub fn present(&mut self) -> Result<(), Error> {
        self.terminal.buf.clear()?;

        self.message_view
            .render(&mut self.terminal, self.terminal_size, self.state.clone())?;
        self.input_view
            .render(&mut self.terminal, self.terminal_size);
        self.indicator
            .render(&mut self.terminal, self.terminal_size);
        if self.message_view.showing_sidebar() {
            self.guild_list
                .render(&mut self.terminal, self.terminal_size, self.state.clone());
        }
        self.terminal.buf.draw()?;
        Ok(())
    }

    pub fn update_size(&mut self) {
        self.terminal_size = self.terminal
            .buf
            .size()
            .expect("Unable to get terminal size");
        self.present().expect("Unable to redraw");
    }
}

impl Drop for View {
    fn drop(&mut self) {
        trace!("Dropping view");
        let _ = self.event_listener_killswitch.send(());
    }
}
