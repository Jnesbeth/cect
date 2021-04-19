use crate::config::Config;
use anyhow::Result;
use async_std::channel::Receiver;
use async_std::sync::{Arc, RwLock};
use async_std::task;
use log::info;
use std::time::{Duration, Instant};

#[derive(Default)]
struct MonitorState {
    sent: u64,
    failed: u64,
}

#[derive(Debug)]
pub enum MonitorMessage {
    Sent,   // A message was succesfully sent.
    Failed, // A send attempt failed.
}

// A progress monitor that displays the following and updates it every N seconds
// (configurable):
//
// a. Number of messages sent so far
// b. Number of messages failed so far
// c. Average time per message so far
pub async fn monitor(config: Arc<Config>, receiver: Receiver<MonitorMessage>) -> Result<()> {
    let state = Arc::new(RwLock::new(MonitorState::default()));
    // Message rate will be computed as sent messages over elapsed time since
    // this timestamp.
    let start = Instant::now();
    {
        // Increment sent and failed counts as informed by the senders.
        let state = state.clone();
        task::spawn(async move {
            while let Ok(message) = receiver.recv().await {
                let mut state = state.write().await;
                match message {
                    MonitorMessage::Sent => state.sent += 1,
                    MonitorMessage::Failed => state.failed += 1,
                };
            }
        });
    }
    // Periodically report the sent and failed counts and sent rate.
    let nominal_loop_duration = Duration::from_secs_f64(config.report_period);
    loop {
        let loop_start = Instant::now();
        let (sent, failed) = {
            let state = state.read().await;
            (state.sent, state.failed)
        };
        let messages_per_second = if sent > 0 {
            format!("{:.3}/s", (sent as f64) / start.elapsed().as_secs_f64())
        } else {
            "N/A".to_string()
        };
        info!(
            "MONITOR sent: {}, failed: {}, rate: {}",
            sent, failed, messages_per_second
        );
        // If the monitor has been informed that all messages were sent, then
        // it should exit so that the program can terminate.
        if sent >= config.num_messages {
            break;
        }
        // Sleep as needed to maintain the desired reporting rate.
        let loop_duration = loop_start.elapsed();
        if loop_duration < nominal_loop_duration {
            task::sleep(nominal_loop_duration - loop_duration).await;
        }
    }
    Ok(())
}
