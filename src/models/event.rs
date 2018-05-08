use serenity::model::channel;
use serenity::model::event::{MessageUpdateEvent, TypingStartEvent};
use serenity::model::id::{ChannelId, MessageId};
use termbuf::termion::event::Key;

use failure::Error;

#[derive(Debug)]
pub enum Event {
    ShutdownAll,
    NewMessage(Box<channel::Message>),
    MessageDelete(ChannelId, MessageId),
    MessageDeleteBulk(ChannelId, Vec<MessageId>),
    MessageUpdateEvent(Box<MessageUpdateEvent>),
    DiscordReady,
    SetChannel(ChannelId),
    Keypress(Key),
    UserMessage(String),
    UserCommand(String),
    UserTyping,
    TypingStart(TypingStartEvent),
    InternalError(Error),
    WindowSizeChange,
}
