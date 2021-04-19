use crate::config::SenderConfig;
use crate::message::Message;
use crate::monitor::MonitorMessage;
use anyhow::Result;
use async_std::channel::{Receiver, Sender};
use async_std::task;
use log::debug;
use rand::distributions::Distribution;
use rand::{thread_rng, Rng};
use rand_distr::Gamma;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

// A sender, who pickups up messages from the producer and simulates sending
// messages by waiting a random period time distributed around a configurable
// mean. The sender also has a configurable failure rate.
//
// The sender notifies the monitor of send failures and successes.
pub async fn sender(
    sender_config: SenderConfig,
    receiver: Receiver<Message>,
    monitor: Sender<MonitorMessage>,
) -> Result<()> {
    // Senders are assigned a unique integer identity.
    static NEXT_SENDER_INDEX: AtomicU64 = AtomicU64::new(0);
    let sender_index = NEXT_SENDER_INDEX.fetch_add(1, Ordering::Relaxed);
    // Gamma distribution yields delays. It gives positive continuous values.
    // A shape of `10.0` gives "nice looking" PDF resembling a bell curve. A
    // scale of `mean / shape` gives a distribution with desired mean.
    const SHAPE: f64 = 10.0;
    let gamma = Gamma::new(SHAPE, sender_config.mean_time / SHAPE).unwrap();
    while let Ok(_message) = receiver.recv().await {
        // Keep trying until the message is sent.
        loop {
            // Delay for random time with configurable mean.
            let delay = {
                let mut rng = thread_rng();
                gamma.sample(&mut rng)
            };
            let delay = Duration::from_secs_f64(delay);
            debug!("sender {} delaying for {:?}.", sender_index, delay);
            task::sleep(delay).await;
            // Fail at configurable rate.
            let failed = {
                let mut rng = thread_rng();
                rng.gen_bool(sender_config.failure_rate)
            };
            // Notify monitor of result.
            if failed {
                debug!("sender {} send failed.", sender_index);
                monitor.send(MonitorMessage::Failed).await?;
            } else {
                monitor.send(MonitorMessage::Sent).await?;
                break;
            }
        }
    }
    Ok(())
}
