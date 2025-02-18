// use redis::PubSub;
use redis::aio::PubSub;
use tokio::sync::mpsc;
use tokio::time::{self, Duration};
// use futures_util::stream::stream::StreamExt;
use tokio_stream::StreamExt;

// pub struct RedisListener {
//     pubsub: PubSub<'static>
// }

// impl RedisListener {
//     pub async fn new(client: &Client) -> redis::RedisResult<Self> {
//         let mut conn: Connection = client.get_async_connection().await?;
//         let pubsub = conn.as_pubsub(); // Borrowing connection
//         Ok(Self { pubsub })
//     }
// }

/// Listens to Redis Pub/Sub channels and forwards messages to the processor.
pub async fn listen_to_channels(
    mut pubsub: PubSub,
    tx: mpsc::Sender<(String, String)>,
) -> redis::RedisResult<()> {
    // loop {
    //     match pubsub.on_message() {
    //         Ok(msg) => {
    //             let channel = msg.get_channel_name().to_string();
    //             let payload: String = msg.get_payload()?;
    //             println!("Received: '{}' on {}", payload, channel);

    //             if tx.send((channel, payload)).await.is_err() {
    //                 eprintln!("Failed to send message to processor");
    //             }
    //         }
    //         Err(e) => {
    //             eprintln!("Error in Pub/Sub: {:?}", e);
    //             time::sleep(Duration::from_secs(1)).await;
    //         }
    //     }
    // }
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
