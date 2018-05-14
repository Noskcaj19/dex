mod event_handler;
pub mod utils;

use std::sync::Arc;
use std::thread;

use serenity::client::bridge::gateway::ShardManager;
use serenity::prelude::*;
use serenity::Client;

use model::Context;

use error;
use failure::Error;

pub struct DiscordClient {
    shard_manager: Arc<Mutex<ShardManager>>,
}

impl DiscordClient {
    pub fn start(context: &Arc<RwLock<Context>>) -> Result<DiscordClient, Error> {
        let handler =
            event_handler::Handler(Arc::new(Mutex::new(context.read().event_channel.clone())));

        let mut client = match Client::new(&context.read().token, handler) {
            Ok(client) => client,
            Err(err) => return Err(error::InternalSerenityError::from(err))?,
        };

        let shard_manager = client.shard_manager.clone();
        thread::spawn(move || {
            client.start_shards(1).unwrap();
        });

        Ok(DiscordClient { shard_manager })
    }

    pub fn shutdown(&self) {
        debug!("Shutting down");
        self.shard_manager.lock().shutdown_all();
    }
}
