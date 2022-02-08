//! We chose to use the Actor design pattern. In order to uniquely own IO ressource
//! like TcpStream.
use crate::message::{Message, Payload};
use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc;
pub mod handle;
pub use crate::error::{Error, ErrorKind, Result};


pub struct Actor {
    receiver: mpsc::Receiver<Message>,
    counter: usize,
    app_id: String,
    node_id: String,
}

impl Actor {
    /// An [`Actor`] is built with a [`Message`] receiver so it
    /// can receive the instruction to do work.
    pub fn new(
        receiver: mpsc::Receiver<Message>,
        app_id: &str,
        node_id: &str,
        counter: usize,
    ) -> Self {
        Self {
            receiver,
            counter,
            app_id: app_id.into(),
            node_id: node_id.into(),
        }
    }

    pub fn counter(&self) -> usize {
        self.counter
    }

    pub fn app_id(&self) -> &str {
        &self.app_id
    }

    pub fn node_id(&self) -> &str {
        &self.node_id
    }

    /// Increment the counter by 1.
    pub fn inc(&mut self) {
        self.counter += 1;
    }

    /// Runs the [`Actor`] asyncronously so it can handle messages concurrently.
    async fn run(&mut self) -> Result<()>{
        while let Some(msg) = self.receiver.recv().await {
            self.handle_message(msg).await?;
        }
        Ok(())
    }

    /// Message handling part. This is where the actual work are done.
    async fn handle_message(&mut self, msg: Message) -> Result<()> {
        match msg {
            Message::SendPayload {
                mut stream,
                respond_to,
            } => {
                self.inc();
                let pl = Payload::from_actor(self);

                stream.write_all(&pl.to_bytes()?).await?;
                stream.flush().await?;

                if let Err(_err) = respond_to.send(self.counter) {
                    return Err(Error::new(ErrorKind::OneShotRecvError));
                }
            }
        }
        Ok(())
    }
}
