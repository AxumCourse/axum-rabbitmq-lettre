use lettre::{message::header::ContentType, Message};

use crate::{Error, Result};

pub struct Email {
    pub from: String,
    pub to: String,
    pub subject: String,
    pub body: String,
}

impl Email {
    pub fn to_message(&self) -> Result<Message> {
        Message::builder()
            .from(self.from.as_str().parse().unwrap())
            .to(self.to.as_str().parse().unwrap())
            .subject(self.subject.as_str())
            .header(ContentType::TEXT_PLAIN)
            .body(self.body.clone())
            .map_err(Error::from)
    }
}
