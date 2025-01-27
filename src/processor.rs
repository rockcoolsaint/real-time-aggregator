use redis::aio::ConnectionManager;
use redis::AsyncCommands;
use tokio::sync::mpsc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Processes messages, aggregates data, and publishes results.
pub async fn process_messages(
    mut manager: ConnectionManager,
    output_channel: String,
    mut rx: mpsc::Receiver<(String, String)>,
    shared_state: Arc<Mutex<HashMap<String, usize>>>,
) -> redis::RedisResult<()> {
    while let Some((channel, _message)) = rx.recv().await {
        let mut state = shared_state.lock().await;
        let count = state.entry(channel.clone()).or_insert(0);
        *count += 1;

        let aggregated_result: Vec<String> = state
            .iter()
            .map(|(chan, cnt)| format!("{}: {}", chan, cnt))
            .collect();

        let result_string = aggregated_result.join(", ");
        println!("Aggregated Result: {}", result_string);

        let mut conn = manager.clone();
        if let Err(e) = conn.publish(output_channel.clone(), result_string).await {
            eprintln!("Failed to publish result: {:?}", e);
        }
    }

    Ok(())
}
