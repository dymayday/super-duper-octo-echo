//! Actor handle part. This is the entry point to use the application's actor model.

use tokio::net::TcpStream;
use tokio::sync::mpsc;
use crate::actor::Actor;
use crate::error::Result;
use crate::message::Message;
use crate::CHAN_SIZE;

/// The Actor handle is the piece that sends the messages
/// in order to interact with the [`Actor`].
pub struct ActorHandle {
    sender: mpsc::Sender<Message>,
}

impl ActorHandle {
    /// Create a new handle and runs the [`Actor`] in the background.
    pub fn new(app_id: &str, node_id: &str, counter: usize) -> Self {
        let (sender, receiver) = mpsc::channel(CHAN_SIZE);
        let mut actor = Actor::new(receiver, app_id, node_id, counter);

        tokio::spawn(async move { actor.run().await });

        Self { sender }
    }

    /// Command to send the [`Message::Payload`] to the address.
    pub async fn send_payload(&self, addr: &str) -> Result<usize> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let stream = TcpStream::connect(addr).await?;

        self.sender
            .send(Message::SendPayload {
                stream,
                respond_to: tx,
            })
            .await?;

        Ok(rx.await?)
    }
}
