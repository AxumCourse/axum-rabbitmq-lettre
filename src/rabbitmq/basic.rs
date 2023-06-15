use lapin::{
    options::{BasicConsumeOptions, BasicPublishOptions, QueueDeclareOptions},
    publisher_confirm::Confirmation,
    types::FieldTable,
    BasicProperties, Connection, ConnectionProperties, ConsumerDelegate,
};

use crate::{Error, Result};

/// 发送消息
pub async fn send(dsn: &str, queue_name: &str, payload: &str) -> Result<Confirmation> {
    // 定义连接属性
    let options = ConnectionProperties::default()
        .with_executor(tokio_executor_trait::Tokio::current())
        .with_reactor(tokio_reactor_trait::Tokio);

    // 连接到服务器
    let conn = Connection::connect(dsn, options)
        .await
        .map_err(Error::from)?;
    // 创建管道
    let chan = conn.create_channel().await.map_err(Error::from)?;

    //定义队列
    let queue = chan
        .queue_declare(
            queue_name,
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .map_err(Error::from)?;

    // 把要发送的数据转成字节数组
    let payload = payload.as_bytes();

    // 发布消息
    chan.basic_publish(
        "",
        queue.name().as_str(),
        BasicPublishOptions::default(),
        payload,
        BasicProperties::default(),
    )
    .await
    .map_err(Error::from)?
    .await
    .map_err(Error::from)
}

/// 接收消息
pub async fn receive<D: ConsumerDelegate + 'static>(
    dsn: &str,
    queue_name: &str,
    tag: &str,
    delegate: D,
) -> Result<()> {
    // 定义连接属性
    let options = ConnectionProperties::default()
        .with_executor(tokio_executor_trait::Tokio::current())
        .with_reactor(tokio_reactor_trait::Tokio);

    // 连接到服务器
    let conn = Connection::connect(dsn, options)
        .await
        .map_err(Error::from)?;
    // 创建管道
    let chan = conn.create_channel().await.map_err(Error::from)?;

    //定义队列
    let queue = chan
        .queue_declare(
            queue_name,
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .map_err(Error::from)?;

    // 定义消息的消费者
    let consumer = chan
        .basic_consume(
            queue.name().as_str(),
            tag,
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .map_err(Error::from)?;
    tracing::debug!("bbb");
    consumer.set_delegate(delegate);
    conn.run().map_err(Error::from)
}

#[cfg(test)]
mod test {
    use dotenv::dotenv;
    use lapin::{message::DeliveryResult, options::BasicAckOptions};

    use crate::{Config, Result};

    const QUEUE_NAME: &str = "AXUM-RS";

    fn get_dsn() -> Result<String> {
        dotenv().ok();
        tracing_subscriber::fmt().init();

        let cfg = Config::from_env()?;
        Ok(cfg.rabbitmq.dsn.clone())
    }

    #[tokio::test]
    async fn test_basic_send() {
        let dsn = get_dsn().unwrap();
        for i in 0..10 {
            let msg = format!("#{} AXUM中文网-axum.rs", i);
            let confirm = super::send(&dsn, QUEUE_NAME, &msg).await;
            match confirm {
                Ok(_) => tracing::info!("[x] 消息已发送成功！{}", msg),
                Err(e) => tracing::error!("{:?}", e),
            };

            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }
    #[tokio::test]
    async fn test_basic_receive() {
        let dsn = get_dsn().unwrap();
        super::receive(
            &dsn,
            QUEUE_NAME,
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
