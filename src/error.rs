//! Error definition of our application.

use serde_json;
use std::error::Error as StdError;
use std::fmt;
use tokio;
use crate::message::Message;

/// Type alias for our own `Error` type.
pub type Result<T> = std::result::Result<T, Error>;

/// An error that ca occur when running the application.
#[derive(Debug)]
pub struct Error(Box<ErrorKind>);

impl Error {
    // Error crate constructor.
    pub(crate) fn new(kind: ErrorKind) -> Self {
        Self(Box::new(kind))
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    SerdeJson(serde_json::Error),
    Io(std::io::Error),
    MpscSendError(tokio::sync::mpsc::error::SendError<Message>),
    OneShotSendError,
    OneShotRecvError,
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match *self.0 {
            ErrorKind::SerdeJson(ref err) => Some(err),
            ErrorKind::Io(ref err) => Some(err),
            ErrorKind::OneShotSendError => None,
            ErrorKind::OneShotRecvError => None,
            ErrorKind::MpscSendError(ref err) => Some(err),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self.0 {
            ErrorKind::SerdeJson(ref err) => err.fmt(f),
            ErrorKind::Io(ref err) => err.fmt(f),
            ErrorKind::OneShotSendError => write!(f, "The receiver dropped."),
            ErrorKind::OneShotRecvError => write!(f, "The channel closed."),
            ErrorKind::MpscSendError(ref err) => err.fmt(f),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::new(ErrorKind::SerdeJson(err))
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::new(ErrorKind::Io(err))
    }
}

impl From<tokio::sync::oneshot::error::RecvError> for Error {
    fn from(_err: tokio::sync::oneshot::error::RecvError) -> Error {
        Error::new(ErrorKind::OneShotRecvError)
    }
}

impl From<tokio::sync::mpsc::error::SendError<Message>> for Error {
    fn from(_err: tokio::sync::mpsc::error::SendError<Message>) -> Error {
        Error::new(ErrorKind::OneShotRecvError)
    }
}
