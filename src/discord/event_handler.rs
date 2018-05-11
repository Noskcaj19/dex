use std::sync::{mpsc, Arc};

use serenity::model::prelude::*;
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

    fn message_update(&self, _: Context, update: event::MessageUpdateEvent) {
        self.0
            .lock()
            .send(MessageUpdateEvent(Box::new(update)))
            .unwrap();
    }

    fn channel_update(&self, _: Context, _: Option<Channel>, _: Channel) {
        self.0.lock().send(ChannelUpdateEvent).unwrap();
    }

    fn typing_start(&self, _: Context, event: TypingStartEvent) {
        self.0.lock().send(TypingStart(event)).unwrap();
    }

    // Called when discord responds READY
    fn ready(&self, _: Context, _: Ready) {
        self.0.lock().send(DiscordReady).unwrap();
    }
}
