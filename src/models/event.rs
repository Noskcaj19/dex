use serenity::model::channel;
use serenity::model::event::MessageUpdateEvent;
use serenity::model::id::{ChannelId, MessageId};
use termion::event::Key;

use failure::Error;

#[derive(Debug)]
pub enum Event {
    ShutdownAll,
    NewMessage(Box<channel::Message>),
    MessageDelete(ChannelId, MessageId),
    MessageDeleteBulk(ChannelId, Vec<MessageId>),
    MessageUpdateEvent(Box<MessageUpdateEvent>),
    DiscordReady,
    Keypress(Key),
    UserMessage(String),
    UserCommand(String),
    UserTyping,
    InternalError(Error),
    WindowSizeChange,
}
