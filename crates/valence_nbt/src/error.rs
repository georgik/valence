extern crate alloc;

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use core::fmt::{self, Display, Formatter};

pub type Result<T, E = Error> = core::result::Result<T, E>;

/// Errors that can occur when encoding or decoding binary NBT.
#[derive(Debug)]
pub struct Error {
    /// Box this to keep the size of `Result<T, Error>` small.
    cause: Box<Cause>,
}

#[derive(Debug)]
enum Cause {
    Io(String),
    Owned(Box<str>),
    Static(&'static str),
}

impl Error {
    /// Creates a new error with an owned string message.
    pub fn new_owned(msg: impl Into<Box<str>>) -> Self {
        Self {
            cause: Box::new(Cause::Owned(msg.into())),
        }
    }

    /// Creates a new error with a static string message.
    pub fn new_static(msg: &'static str) -> Self {
        Self {
            cause: Box::new(Cause::Static(msg)),
        }
    }

    /// Creates a new error from an I/O error description.
    pub fn new_io(msg: impl ToString) -> Self {
        Self {
            cause: Box::new(Cause::Io(msg.to_string())),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &*self.cause {
            Cause::Io(e) => write!(f, "I/O error: {}", e),
            Cause::Owned(msg) => write!(f, "{}", msg),
            Cause::Static(msg) => write!(f, "{}", msg),
        }
    }
}

impl From<&'static str> for Error {
    fn from(msg: &'static str) -> Self {
        Self::new_static(msg)
    }
}

impl From<String> for Error {
    fn from(msg: String) -> Self {
        Self::new_io(msg)
    }
}
