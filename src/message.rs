//! Definition of the messages that will be sent between the agent in the system.

use crate::actor::Actor;
use serde::{Deserialize, Serialize};
use serde_json;
use tokio::net::TcpStream;
use tokio::sync::oneshot;

/// Using an actor model, everything that happens is by the use of messages.
/// No direct operation are allowed.
#[derive(Debug)]
pub enum Message {
    SendPayload {
        stream: TcpStream,
        respond_to: oneshot::Sender<usize>,
    },
}

/// The actual data structure that is sent to the server.
/// We use a hash so we can ensure by some relevent process
/// that a node id is unique in the entire network.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Payload {
    counter: usize,
    app_id: String,
    node_id: String,
}

impl Payload {
    /// Keep in mind here that `app id` is the same for the master
    /// and the fail over actor.
    pub fn new(counter: usize, app_id: &str, node_id: &str) -> Self {
        Self {
            counter,
            app_id: app_id.into(),
            node_id: node_id.into(),
        }
    }

    /// Helper function for convience reason.
    pub fn from_actor(actor: &Actor) -> Self {
        Self {
            counter: actor.counter(),
            app_id: actor.app_id().into(),
            node_id: actor.node_id().into(),
        }
    }

    /// Serialize a [`Payload`] into a vector of bytes.
    pub fn to_bytes(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(&self)
    }

    /// Deserriaze a [`Payload`] from a vector of bytes.
    pub fn from_bytes(payload: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(payload)
    }
}

#[test]
fn test_load() {
    let pl = Payload::new(0, "Master", "34567");
    assert_eq!(pl, Payload::from_bytes(&pl.to_bytes().unwrap()).unwrap());
}
