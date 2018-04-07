use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

pub struct Handler;

impl EventHandler for Handler {
    fn message(&self, _: Context, msg: Message) {
        println!(">> {}", msg.content);
    }

    // Called when discord responds READY
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} connected!", ready.user.name);
    }
}
