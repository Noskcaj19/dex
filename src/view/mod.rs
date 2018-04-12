mod terminal;
mod messages;

use std::sync::mpsc::{self, Sender, SyncSender};
use std::sync::Arc;

use models::event::Event;

pub struct View {
    pub terminal: Arc<terminal::Terminal>,
    event_channel: Sender<Event>,
    event_listener_killswitch: SyncSender<()>,
}

impl View {
    pub fn new(event_channel: Sender<Event>) -> View {
        let terminal = terminal::Terminal::new().unwrap();

        let (killswitch_tx, killswitch_rx) = mpsc::sync_channel(0);
        terminal.listen(event_channel.clone(), killswitch_rx);
        let terminal = Arc::new(terminal);

        View {
            terminal,
            event_channel,
            event_listener_killswitch: killswitch_tx,
        }
    }
}

impl Drop for View {
    fn drop(&mut self) {
        trace!("Dropping view");
        let _ = self.event_listener_killswitch.send(());
    }
}
