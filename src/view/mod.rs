mod guild_list;
mod indicator;
mod input;
mod messages;
mod terminal;

use std::sync::mpsc::{self, SyncSender};
use std::sync::Arc;

use model::Context;

use serenity::prelude::RwLock;

use termbuf;

use failure::Error;

pub struct View {
    pub terminal: terminal::Terminal,
    event_listener_killswitch: SyncSender<()>,
    pub message_view: messages::Messages,
    pub input_view: input::Input,
    pub terminal_size: termbuf::TermSize,
    pub indicator: indicator::Indicator,
    pub guild_list: guild_list::GuildList,
    pub context: Arc<RwLock<Context>>,
}

impl View {
    pub fn new(context: &Arc<RwLock<Context>>) -> View {
        let locked_ctx = context.read();

        let terminal = terminal::Terminal::new().unwrap();

        let terminal_size = terminal.buf.size().expect("Unable to get size");

        let (killswitch_tx, killswitch_rx) = mpsc::sync_channel(0);
        terminal.listen(locked_ctx.event_channel.clone(), killswitch_rx);

        let message_view = messages::Messages::new(locked_ctx.timestamp_fmt.clone(), false);
        let input_view = input::Input::new(locked_ctx.event_channel.clone());
        let indicator = indicator::Indicator::new(locked_ctx.event_channel.clone());
        let guild_list = guild_list::GuildList::new();

        View {
            terminal,
            event_listener_killswitch: killswitch_tx,
            message_view,
            input_view,
            terminal_size,
            indicator,
            guild_list,
            context: context.clone(),
        }
    }

    pub fn present(&mut self) -> Result<(), Error> {
        self.terminal.buf.clear()?;

        self.message_view.render(
            &mut self.terminal,
            self.terminal_size,
            &self.context.clone(),
        )?;
        self.input_view
            .render(&mut self.terminal, self.terminal_size);
        self.indicator
            .render(&mut self.terminal, self.terminal_size);
        if self.message_view.showing_sidebar() {
            self.guild_list.render(
                &mut self.terminal,
                self.terminal_size,
                &self.context.clone(),
            );
        }
        self.terminal.buf.draw()?;
        Ok(())
    }

    pub fn update_size(&mut self) {
        self.terminal_size = self
            .terminal
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
