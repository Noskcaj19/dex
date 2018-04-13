use serenity::model::channel;
use termion::event::Key;

#[derive(Debug)]
pub enum Event {
    ShutdownAll,
    NewMessage(channel::Message),
    DiscordReady,
    Keypress(Key),
    UserCommand(String),
}
