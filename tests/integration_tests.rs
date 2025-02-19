#[tokio::test]
async fn test_redis_pubsub() -> redis::RedisResult<()> {
    use redis::AsyncCommands;
    // use redis::aio::PubSub;
    use tokio::time::{timeout, Duration};
    use real_time_aggregator::config::RedisConfig;
    use futures::stream::StreamExt;

    let config = RedisConfig::default();
    let client = redis::Client::open(config.url.clone())?;
    // let mut pubsub_conn = client.get_multiplexed_async_connection().await.expect("Failed to connect");
    // let mut pubsub = pubsub_conn.into_pubsub();

    let mut pubsub = client.get_async_pubsub().await.expect("Failed to get connection");

    pubsub.subscribe("test_channel").await.expect("Failed to subscribe");

    
    // Publish a message
    let mut conn = client.get_multiplexed_async_connection().await?;
    let _: () = conn.publish("test_channel", "Hello, test!").await.expect("Failed to publish");

    // Read message with timeout
    let msg = timeout(Duration::from_secs(3), pubsub.on_message().next())
        .await
        .expect("Timeout waiting for message")
        .expect("Failed to receive message");

    let payload: String = msg.get_payload().expect("Failed to get payload");
    assert_eq!(payload, "Hello, test!");

    Ok(())
}
