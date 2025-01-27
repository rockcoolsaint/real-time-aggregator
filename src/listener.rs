use redis::AsyncCommands;
use redis::aio::{MultiplexedConnection, PubSub};
use tokio::sync::mpsc;
use tokio::time::{self, Duration};

/// Creates a Pub/Sub connection and listens to multiple Redis channels
pub async fn listen_to_channels(
    redis_url: &str,
    channels: Vec<String>,
    tx: mpsc::Sender<(String, String)>,
) -> redis::RedisResult<()> {
    // Create a Redis client
    let client = redis::Client::open(redis_url)?;
    
    // Get a multiplexed connection
    let mut conn: MultiplexedConnection = client.get_multiplexed_async_connection().await?;

    // Create a PubSub instance from the multiplexed connection
    let mut pubsub = conn.pubsub();

    // Subscribe to all channels
    for channel in &channels {
        pubsub.subscribe(channel).await?;
        println!("Subscribed to {}", channel);
    }

    // Listen for messages
    loop {
        match pubsub.on_message().await {
            Ok(msg) => {
                let channel = msg.get_channel_name().to_string();
                let payload: String = msg.get_payload()?;
                println!("Received: '{}' on {}", payload, channel);

                if tx.send((channel, payload)).await.is_err() {
                    eprintln!("Failed to send message to processor");
                }
            }
            Err(e) => {
                eprintln!("Error in Pub/Sub: {:?}", e);
                time::sleep(Duration::from_secs(1)).await;
            }
        }
    }
}
