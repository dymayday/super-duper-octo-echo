//! Nothing much here except the needed exports by the project architecture.

pub mod actor;
pub mod error;
pub mod message;
pub use actor::handle::ActorHandle;
pub use actor::Actor;

pub const CHAN_SIZE: usize = 8;

// TODO:
// - better documentation
