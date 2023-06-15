use lapin::{
    options::{
        BasicConsumeOptions, BasicPublishOptions, ExchangeDeclareOptions, QueueBindOptions,
        QueueDeclareOptions,
    },
    publisher_confirm::Confirmation,
    types::FieldTable,
    BasicProperties, Connection, ConnectionProperties, ConsumerDelegate,
};

use crate::{Error, Result};

pub async fn send(
    dsn: &str,
    exchange: &str,
    queue_name: &str,
    routing_key: &str,
    payload: &str,
) -> Result<Confirmation> {
    let options = ConnectionProperties::default()
        .with_executor(tokio_executor_trait::Tokio::current())
        .with_reactor(tokio_reactor_trait::Tokio);

    let conn = Connection::connect(dsn, options)
        .await
        .map_err(Error::from)?;
    let chan = conn.create_channel().await.map_err(Error::from)?;

    chan.exchange_declare(
        exchange,
        lapin::ExchangeKind::Topic,
        ExchangeDeclareOptions::default(),
        FieldTable::default(),
    )
    .await
    .map_err(Error::from)?;

    let queue = chan
        .queue_declare(
            queue_name,
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .map_err(Error::from)?;

    chan.queue_bind(
        queue.name().as_str(),
        exchange,
        routing_key,
        QueueBindOptions::default(),
        FieldTable::default(),
    )
    .await
    .map_err(Error::from)?;

    let payload = payload.as_bytes();

    chan.basic_publish(
        exchange,
        routing_key,
        BasicPublishOptions::default(),
        payload,
        BasicProperties::default(),
    )
    .await
    .map_err(Error::from)?
    .await
    .map_err(Error::from)
}

pub async fn receive<D: ConsumerDelegate + 'static>(
    dsn: &str,
    exchange: &str,
    queue_name: &str,
    routing_key: &str,
    tag: &str,
    delegate: D,
) -> Result<()> {
    let options = ConnectionProperties::default()
        .with_executor(tokio_executor_trait::Tokio::current())
        .with_reactor(tokio_reactor_trait::Tokio);

    let conn = Connection::connect(dsn, options)
        .await
        .map_err(Error::from)?;
    let chan = conn.create_channel().await.map_err(Error::from)?;

    chan.exchange_declare(
        exchange,
        lapin::ExchangeKind::Topic,
        ExchangeDeclareOptions::default(),
        FieldTable::default(),
    )
    .await
    .map_err(Error::from)?;

    let queue = chan
        .queue_declare(
            queue_name,
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .map_err(Error::from)?;

    chan.queue_bind(
        queue.name().as_str(),
        exchange,
        routing_key,
        QueueBindOptions::default(),
        FieldTable::default(),
    )
    .await
    .map_err(Error::from)?;

    let consumer = chan
        .basic_consume(
            queue.name().as_str(),
            tag,
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .map_err(Error::from)?;

    consumer.set_delegate(delegate);

    conn.run().map_err(Error::from)
}

#[cfg(test)]
mod test {
    use dotenv::dotenv;
    use lapin::{message::DeliveryResult, options::BasicAckOptions};

    use crate::{Config, Result};

    const QUEUE_NAME: &str = "AXUM-RS";
    const EXCHANGE_NAME: &str = "USER-REGISTER";
    const ROUTING_KEY: &str = "AXUM-RS";

    fn get_dsn() -> Result<String> {
        dotenv().ok();
        tracing_subscriber::fmt().init();

        let cfg = Config::from_env()?;
        Ok(cfg.rabbitmq.dsn.clone())
    }

    #[tokio::test]
    async fn test_topic_send() {
        let dsn = get_dsn().unwrap();
        for i in 0..10 {
            let msg = format!("#{} AXUM中文网-axum.rs", i);
            let confirm = super::send(&dsn, EXCHANGE_NAME, QUEUE_NAME, ROUTING_KEY, &msg).await;
            match confirm {
                Ok(_) => tracing::info!("[x] 消息已发送成功！{}", msg),
                Err(e) => tracing::error!("{:?}", e),
            };

            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }
    #[tokio::test]
    async fn test_topic_receive() {
        let dsn = get_dsn().unwrap();
        super::receive(
            &dsn,
            EXCHANGE_NAME,
            QUEUE_NAME,
            ROUTING_KEY,
            "TESTER",
            move |delivery: DeliveryResult| async move {
                tracing::debug!("aaa");
                let delivery = match delivery {
                    Ok(Some(delivery)) => delivery,
                    Ok(None) => {
                        tracing::error!("None ");
                        return;
                    }
                    Err(err) => {
                        tracing::error!("Failed to consume queue message {}", err);
                        return;
                    }
                };

                let message = String::from_utf8_lossy(&delivery.data);
                tracing::info!("Received a message: {}", message);

                delivery.ack(BasicAckOptions::default()).await.unwrap();
            },
        )
        .await
        .unwrap();
    }
}
