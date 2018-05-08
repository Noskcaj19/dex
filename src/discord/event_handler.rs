use std::sync::{mpsc, Arc};

use serenity::model::channel::Message;
use serenity::model::event::{MessageUpdateEvent, TypingStartEvent};
use serenity::model::gateway::Ready;
use serenity::model::id::{ChannelId, MessageId};
use serenity::prelude::*;

use models::event::Event::{self, *};

pub struct Handler(pub Arc<Mutex<mpsc::Sender<Event>>>);

impl EventHandler for Handler {
    // Called when a message is received
    fn message(&self, _: Context, msg: Message) {
        self.0.lock().send(NewMessage(Box::new(msg))).unwrap();
    }

    fn message_delete(&self, _: Context, channel: ChannelId, message: MessageId) {
        self.0.lock().send(MessageDelete(channel, message)).unwrap()
    }

    fn message_delete_bulk(&self, _: Context, channel: ChannelId, messages: Vec<MessageId>) {
        self.0
            .lock()
            .send(MessageDeleteBulk(channel, messages))
            .unwrap()
    }

    fn message_update(&self, _: Context, update: MessageUpdateEvent) {
        self.0
            .lock()
            .send(MessageUpdateEvent(Box::new(update)))
            .unwrap();
    }

    fn typing_start(&self, _: Context, event: TypingStartEvent) {
        self.0.lock().send(TypingStart(event)).unwrap();
    }

    // Called when discord responds READY
    fn ready(&self, _: Context, _: Ready) {
        self.0.lock().send(DiscordReady).unwrap();
    }
}
