use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use communication::ChannelMessage::NewMessage;

pub struct Handler;

impl EventHandler for Handler {
    fn message(&self, _: Context, msg: Message) {
        ::MESSAGE_CHANNEL
            .lock()
            .unwrap()
            .send(NewMessage(msg))
            .unwrap();
    }

    // Called when discord responds READY
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} connected!", ready.user.name);
    }
}
