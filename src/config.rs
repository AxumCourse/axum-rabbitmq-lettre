use serde::{Deserialize, Serialize};

use crate::{Error, Result};

#[derive(Deserialize, Clone)]
pub struct WebConfig {
    pub addr: String,
}

#[derive(Deserialize, Clone)]
pub struct RabbitMQConfig {
    pub dsn: String,
    pub exchange_name: String,
    pub queue_name: String,
    pub routing_key: String,
}

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct EmailConfig {
    pub username: String,
    pub password: String,
    pub host: String,
}

#[derive(Deserialize, Clone)]
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
