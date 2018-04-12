mod event_handler;

use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::thread;

use serenity::client::bridge::gateway::ShardManager;
use serenity::prelude::*;
use serenity::Client;

use models::event::Event;

use errors;
use failure::Error;

pub struct DiscordClient {
    // client: Mutex<Client>,
    shard_manager: Arc<Mutex<ShardManager>>,
}

impl DiscordClient {
    pub fn start(token: &str, event_channel: Sender<Event>) -> Result<DiscordClient, Error> {
        let handler = event_handler::Handler(Arc::new(Mutex::new(event_channel)));

        let mut client = match Client::new(token, handler) {
            Ok(client) => client,
            Err(err) => return Err(errors::InternalSerenityError::from(err))?,
        };

        let shard_manager = client.shard_manager.clone();
        thread::spawn(move || {
            client.start();
        });

        Ok(DiscordClient {
            shard_manager,
        })
    }

    pub fn shutdown(&self) {
        self.shard_manager.lock().shutdown_all();
    }
}
