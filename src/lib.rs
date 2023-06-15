mod config;
mod err;
pub mod rabbitmq;

pub use crate::config::*;
pub use err::Error;
pub use err::Kind as ErrorKind;

pub type Result<T> = std::result::Result<T, crate::Error>;
