mod listener;
mod processor;
mod config;

use listener::listen_to_channels;
use processor::process_messages;
use config::RedisConfig;
use tokio::{signal, sync::mpsc, task};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> redis::RedisResult<()> {
    // Load Redis configuration
    let config = RedisConfig::default();
    let client = redis::Client::open(config.url.clone())?;
    // let mut conn = client.get_multiplexed_async_connection().await?;

    // Input and output channels
    let input_channels = config.input_channels.clone();
    let output_channel = config.output_channel.clone();

    let mut pubsub = client.get_async_pubsub().await?;
    // Create a PubSub connection
    for channel in &input_channels {
        pubsub.subscribe(channel).await?;
    }

    // Create communication channel
    let (tx, rx) = mpsc::channel::<(String, String)>(100);

    // Shared state for aggregation
    let shared_state = Arc::new(Mutex::new(HashMap::new()));

    // Spawn tasks
    let listener_task = task::spawn(listen_to_channels(pubsub, tx));
    let processor_task = task::spawn(process_messages(client, output_channel, rx, shared_state));

    // Graceful shutdown
    tokio::select! {
        _ = signal::ctrl_c() => println!("Shutting down..."),
        result = listener_task => if let Err(e) = result {
            eprintln!("Listener task error: {:?}", e);
        },
        result = processor_task => if let Err(e) = result {
            eprintln!("Processor task error: {:?}", e);
        },
    }

    Ok(())
}
