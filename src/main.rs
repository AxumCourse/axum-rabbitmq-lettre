use axum_rabbitmq_lettre::{rabbitmq::basic, Config};
use dotenv::dotenv;
use lapin::{message::DeliveryResult, options::BasicAckOptions};

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt().init();

    let cfg = Config::from_env().unwrap();

    basic::receive(
        &cfg.rabbitmq.dsn,
        "AXUM-RS",
        "AXUM-RS-CONSUMER",
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
