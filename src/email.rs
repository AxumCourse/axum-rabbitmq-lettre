use lettre::{
    transport::smtp::authentication::Credentials, transport::smtp::response::Response,
    AsyncSmtpTransport, AsyncTransport, SmtpTransport, Tokio1Executor, Transport,
};

use crate::{model, EmailConfig, Error, Result};

/// 同步发送
pub fn sync_send(cfg: &EmailConfig, m: &model::email::Email) -> Result<Response> {
    let message = m.to_message()?;

    let creds = Credentials::new(cfg.username.clone(), cfg.password.clone());

    let mailer = SmtpTransport::relay(&cfg.host)
        .map_err(Error::from)?
        .credentials(creds)
        .build();

    mailer.send(&message).map_err(Error::from)
}

/// 异步发送
pub async fn send(cfg: EmailConfig, m: model::email::Email) -> Result<Response> {
    let message = m.to_message()?;

    let creds = Credentials::new(cfg.username.clone(), cfg.password.clone());

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&cfg.host)
        .map_err(Error::from)?
        .credentials(creds)
        .build();

    mailer.send(message).await.map_err(Error::from)
}

#[cfg(test)]
mod test {
    use dotenv::dotenv;

    use crate::{model, Config, EmailConfig, Result};

    fn get_cfg() -> Result<EmailConfig> {
        dotenv().ok();
        tracing_subscriber::fmt().init();

        let cfg = Config::from_env()?;
        Ok(cfg.email)
    }
    #[test]
    fn test_sync_send_email() {
        let cfg = get_cfg().unwrap();
        let m = model::email::Email {
            from: format!("{}", &cfg.username),
            to: format!("team@axum.rs"),
            subject: format!("试试同步发送"),
            body: format!("你好呀，这是用lettre同步发送的邮件！"),
        };
        let resp = super::sync_send(&cfg, &m).unwrap();
        tracing::info!("{:?}", resp);
    }
    #[tokio::test]
    async fn test_async_send_email() {
        let cfg = get_cfg().unwrap();
        let m = model::email::Email {
            from: format!("{}", &cfg.username),
            to: format!("team@axum.rs"),
            subject: format!("试试异步发送"),
            body: format!("你好呀，这是用lettre异步发送的邮件！"),
        };
        let resp = super::send(cfg, m).await.unwrap();
        tracing::info!("{:?}", resp);
    }
}
