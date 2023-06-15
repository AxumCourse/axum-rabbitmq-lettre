mod config;
pub mod email;
mod err;
pub mod model;
pub mod rabbitmq;

pub use crate::config::*;
pub use err::Error;
pub use err::Kind as ErrorKind;

pub type Result<T> = std::result::Result<T, crate::Error>;
