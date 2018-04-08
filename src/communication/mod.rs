use serenity::model::channel;

pub enum ChannelMessage {
    ShutdownAll,
    NewMessage(channel::Message),
}
