#[derive(Debug, Clone)]
pub struct RedisConfig {
    pub url: String,
    pub input_channels: Vec<String>,
    pub output_channel: String,
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            url: "redis://127.0.0.1:6379".to_string(),
            input_channels: vec!["inputA".to_string(), "inputB".to_string(), "inputC".to_string()],
            output_channel: "outputChannel".to_string(),
        }
    }
}
