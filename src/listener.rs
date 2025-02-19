// use redis::PubSub;
use redis::aio::PubSub;
use tokio::sync::mpsc;
use tokio_stream::StreamExt;

/// Listens to Redis Pub/Sub channels and forwards messages to the processor.
pub async fn listen_to_channels(
    mut pubsub: PubSub,
    tx: mpsc::Sender<(String, String)>,
) -> redis::RedisResult<()> {
    let mut message_stream = pubsub.on_message();

    while let Some(msg) = message_stream.next().await {
        let channel = msg.get_channel_name().to_string();
        let payload: String = msg.get_payload()?;
        println!("Received: '{}' on {}", payload, channel);

        if tx.send((channel, payload)).await.is_err() {
            eprintln!("Failed to send message to processor");
        }
    }

    Ok(())
}
