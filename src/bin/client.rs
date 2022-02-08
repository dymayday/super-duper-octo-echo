//! The client sends a payload every seconds to the server.
//! A simple fail over system is used in order to maximize runtime.

#[macro_use]
extern crate log;
use cio::ActorHandle;
use tokio;
use tokio::time;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_id = "Cio".into();
    let addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:7788".into());
    let node_id = &std::process::id().to_string();

    env_logger::builder().try_init()?;

    let mut interval = time::interval(time::Duration::from_millis(1_000));

    let mut counter = 0;
    let mut actor = ActorHandle::new(app_id, node_id, counter);

    // Used to exit the program at the second event of ctrl-c.
    let mut failed_over_already = false;

    loop {
        // Using select let us handle an interuption concurrently with
        // the main working loop.
        tokio::select! {
            // Main working loop.
            _ = async {
                loop {
                    counter = match actor.send_payload(&addr).await {
                        Ok(c) => c,
                        Err(err) => {
                            error!("Fail to send paylaod : `{}`", err);
                            counter
                        }
                    };
                    interval.tick().await;
                }
            } => {}
            // Interupting the work.
            _ = tokio::signal::ctrl_c() => {


                // Switching to the fail over Actor.
                actor = match failed_over_already {
                    false => {
                        info!(">> Ctrl-C received. Switching to fail over actor.");
                        failed_over_already = true;
                        drop(actor);
                        ActorHandle::new(app_id, "Fail over.", counter)
                    }
                    true => {
                        warn!(">> Ctrl-C received again. Exiting...");
                        return Ok(());
                    }
                }
            }
        }
    }
}
