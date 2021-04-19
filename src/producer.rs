use crate::config::Config;
use crate::message::Message;
use anyhow::Result;
use async_std::channel::Sender;
use std::sync::Arc;

// A producer that that generates a configurable number of messages (default
// 1000) to random phone number. Each message contains up to 100 random
// characters.
pub async fn producer(config: Arc<Config>, sender: Sender<Message>) -> Result<()> {
    for _ in 0..config.num_messages {
        let message = Message::random();
        sender.send(message).await?;
    }
    Ok(())
}
