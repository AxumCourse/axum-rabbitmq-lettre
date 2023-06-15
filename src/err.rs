use axum::response::IntoResponse;

#[derive(Debug)]
pub enum Kind {
    Config,
    RabbitMQ,
}

#[derive(Debug)]
pub struct Error {
    pub kind: Kind,
    pub message: String,
    pub cause: Option<Box<dyn std::error::Error>>,
}

impl Error {
    pub fn new(kind: Kind, message: String, cause: Option<Box<dyn std::error::Error>>) -> Self {
        Self {
            kind,
            message,
            cause,
        }
    }

    pub fn from_str(kind: Kind, msg: &str) -> Self {
        Self::new(kind, msg.to_string(), None)
    }

    pub fn with_cause(kind: Kind, cause: Box<dyn std::error::Error>) -> Self {
        Self::new(kind, cause.to_string(), Some(cause))
    }
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<config::ConfigError> for Error {
    fn from(e: config::ConfigError) -> Self {
        Self::with_cause(Kind::Config, Box::new(e))
    }
}

impl From<lapin::Error> for Error {
    fn from(e: lapin::Error) -> Self {
        Self::with_cause(Kind::RabbitMQ, Box::new(e))
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        self.message.into_response()
    }
}
