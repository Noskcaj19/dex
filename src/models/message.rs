use serenity::model::channel;

#[derive(Clone, Debug)]
pub enum MessageItem {
    DiscordMessage(Box<channel::Message>),
}
