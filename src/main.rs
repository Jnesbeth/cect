use crate::config::{Config, SenderConfig};
use crate::monitor::monitor;
use crate::producer::producer;
use crate::sender::sender;
use anyhow::Result;
use async_std::channel::{bounded, unbounded};
use async_std::task;
use futures_util::future::try_join_all;
use log::info;
use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;
use structopt::StructOpt;

mod config;
mod message;
mod monitor;
mod producer;
mod sender;

#[derive(StructOpt)]
struct Opts {
    /// Path of YAML configuration file.
    config_path: Option<PathBuf>,
}

#[async_std::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;
    env_logger::init();
    let opts = Opts::from_args();
    let config = if let Some(config_path) = &opts.config_path {
        let mut config_file = File::open(config_path)?;
        serde_yaml::from_reader(&mut config_file)?
    } else {
        Config::default()
    };
    let config = Arc::new(config);
    info!("{:?}", config);

    // Compute and display the expected successful message transmission rate.
    compute_expected_rate(&config.sender_configs);

    // Create channel for the producer to send to the senders.
    let (message_sender, message_receiver) = bounded(config.sender_configs.len() * 4);
    // Create channel for the senders to send to the monitor.
    let (monitor_sender, monitor_receiver) = unbounded();

    // Collect join handles for all async tasks so they can be awaited together.
    let mut handles = Vec::new();
    handles.push(task::spawn(producer(config.clone(), message_sender)));
    handles.push(task::spawn(monitor(config.clone(), monitor_receiver)));
    for sender_config in &config.sender_configs {
        handles.push(task::spawn(sender(
            sender_config.clone(),
            message_receiver.clone(),
            monitor_sender.clone(),
        )));
    }
    // Any async task may fail with error or all of them may complete successfully.
    try_join_all(handles).await?;
    Ok(())
}

// Given the configuration, compute the expected message rate. The simulation
// should exhibit a similar rate, but it is stochastic.
fn compute_expected_rate(sender_configs: &[SenderConfig]) {
    // Total messages per second is sum per sender.
    let mut total_messages_per_second = 0.0;
    for sender_config in sender_configs {
        // On average, it takes how many attempts to succesfully send? Use the
        // expected value of the geometric distribution:
        let attempts_per_send = 1.0 / (1.0 - sender_config.failure_rate);
        let messages_per_second = 1.0 / (sender_config.mean_time * attempts_per_send);
        total_messages_per_second += messages_per_second;
    }
    info!("Expected rate {:.3}/s", total_messages_per_second);
}
