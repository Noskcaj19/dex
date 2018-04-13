mod input;
mod messages;
mod terminal;

use std::io::{self, Write};
use std::sync::mpsc::{self, Sender, SyncSender};

use models::event::Event;
use models::preferences::Preferences;

pub struct View {
    pub terminal: terminal::Terminal,
    event_channel: Sender<Event>,
    event_listener_killswitch: SyncSender<()>,
    pub message_view: messages::Messages,
    pub input_view: input::Input,
}

impl View {
    pub fn new(preferences: Preferences, event_channel: Sender<Event>) -> View {
        let terminal = terminal::Terminal::new().unwrap();

        let (killswitch_tx, killswitch_rx) = mpsc::sync_channel(0);
        terminal.listen(event_channel.clone(), killswitch_rx);

        let message_view = messages::Messages::new(preferences.timestamp_fmt());
        let input_view = input::Input::new(event_channel.clone());

        View {
            terminal,
            event_channel,
            event_listener_killswitch: killswitch_tx,
            message_view,
            input_view,
        }
    }

    pub fn present(&mut self) -> Result<(), io::Error> {
        write!(self.terminal, "{}", ::termion::clear::All)?;
        let terminal_size = self.terminal.size();

        self.message_view.render(&mut self.terminal, terminal_size)?;
        self.input_view.render(&mut self.terminal, terminal_size);
        self.terminal.flush()
    }
}

impl Drop for View {
    fn drop(&mut self) {
        trace!("Dropping view");
        let _ = self.event_listener_killswitch.send(());
    }
}
