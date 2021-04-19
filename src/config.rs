use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// Simulation configuration.
pub struct Config {
    #[serde(default = "Config::default_producer_message_quantity")]
    /// Total quantity of messages to send.
    pub num_messages: u64,
    #[serde(default, rename = "senders")]
    /// Vector of sender configurations.
    pub sender_configs: Vec<SenderConfig>,
    #[serde(default = "Config::default_report_period")]
    /// Period, in seconds, between monitor reports.
    pub report_period: f64,
}

impl Config {
    fn default_producer_message_quantity() -> u64 {
        1000
    }

    fn default_report_period() -> f64 {
        5.0
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            num_messages: Config::default_producer_message_quantity(),
            report_period: Config::default_report_period(),
            sender_configs: vec![
                SenderConfig {
                    mean_time: SenderConfig::default_mean_time(),
                    failure_rate: SenderConfig::default_failure_rate(),
                };
                5
            ],
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
/// Configuration of a single sender.
pub struct SenderConfig {
    #[serde(default = "SenderConfig::default_mean_time")]
    /// Mean time, in seconds, between message send attempts.
    pub mean_time: f64,
    #[serde(default = "SenderConfig::default_failure_rate")]
    /// Rate, between 0 and 1, at which message send attempts fail.
    pub failure_rate: f64,
}

impl SenderConfig {
    fn default_mean_time() -> f64 {
        1.0
    }

    fn default_failure_rate() -> f64 {
        0.01
    }
}

#[test]
fn test_default_config() {
    let config = Config::default();
    assert_eq!(config.num_messages, 1000);
    assert_eq!(config.report_period, 5.0);
    assert_eq!(config.sender_configs.len(), 5);
    let sender_config = &config.sender_configs[0];
    assert_eq!(sender_config.mean_time, 1.0);
    assert_eq!(sender_config.failure_rate, 0.01);
}
