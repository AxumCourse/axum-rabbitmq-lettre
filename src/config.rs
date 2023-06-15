use serde::Deserialize;

use crate::{Error, Result};

#[derive(Deserialize)]
pub struct WebConfig {
    pub addr: String,
}

#[derive(Deserialize)]
pub struct RabbitMQConfig {
    pub dsn: String,
}

#[derive(Deserialize)]
pub struct EmailConfig {
    pub username: String,
    pub password: String,
    pub host: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub web: WebConfig,
    pub rabbitmq: RabbitMQConfig,
    pub email: EmailConfig,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        config::Config::builder()
            .add_source(config::Environment::default())
            .build()
            .map_err(Error::from)?
            .try_deserialize()
            .map_err(Error::from)
    }
}
