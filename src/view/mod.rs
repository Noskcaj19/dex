mod messages;
mod terminal;

use std::io::Write;
use std::sync::mpsc::{self, Sender, SyncSender};
use std::sync::Arc;

use models::event::Event;
use models::preferences::Preferences;

pub struct View {
    pub terminal: terminal::Terminal,
    event_channel: Sender<Event>,
    event_listener_killswitch: SyncSender<()>,
    message_view: messages::Messages,
}

impl View {
    pub fn new(preferences: Preferences, event_channel: Sender<Event>) -> View {
        let terminal = terminal::Terminal::new().unwrap();

        let (killswitch_tx, killswitch_rx) = mpsc::sync_channel(0);
        terminal.listen(event_channel.clone(), killswitch_rx);

        let message_view = messages::Messages::new(preferences.timestamp_fmt());

        View {
            terminal,
            event_channel,
            event_listener_killswitch: killswitch_tx,
            message_view,
        }
    }

    pub fn present(&mut self) {
        write!(self.terminal, "{}", ::termion::clear::All).unwrap();
        self.terminal.flush().unwrap();
    }
}

impl Drop for View {
    fn drop(&mut self) {
        trace!("Dropping view");
        let _ = self.event_listener_killswitch.send(());
    }
}
