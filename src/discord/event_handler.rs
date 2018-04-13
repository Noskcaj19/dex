use std::sync::{mpsc, Arc};

use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use models::event::Event::{self, *};

pub struct Handler(pub Arc<Mutex<mpsc::Sender<Event>>>);

impl EventHandler for Handler {
    // Called when a message is received
    fn message(&self, _: Context, msg: Message) {
        self.0.lock().send(NewMessage(msg)).unwrap();
    }

    // Called when discord responds READY
    fn ready(&self, _: Context, _: Ready) {
        self.0.lock().send(DiscordReady).unwrap();
    }
}
