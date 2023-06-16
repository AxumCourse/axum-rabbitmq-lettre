use std::sync::Arc;

use axum::{
    routing::{get, post},
    Extension, Router,
};
use axum_rabbitmq_lettre::{
    email, handler,
    model::{self, state::AppState},
    rabbitmq::topic,
    Config,
};
use dotenv::dotenv;
use lapin::{message::DeliveryResult, options::BasicAckOptions};

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt().init();

    let cfg = Config::from_env().unwrap();

    tokio::spawn(send_active_code(cfg.clone()));

    let addr = cfg.web.addr.clone();

    let app = Router::new()
        .route("/", get(handler::register_ui))
        .route("/register", post(handler::register))
        .route("/active", get(handler::active_ui).post(handler::active))
        .layer(Extension(Arc::new(AppState { cfg })));

    tracing::info!("WEB运行于：{}", &addr);

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn send_active_code(cfg: Config) {
    topic::receive(
        &cfg.rabbitmq.dsn,
        &cfg.rabbitmq.exchange_name,
        &cfg.rabbitmq.queue_name,
        &cfg.rabbitmq.routing_key,
        "MAIL",
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

            let ac: model::user::ActiveCode = serde_json::from_str(&message).unwrap();
            let from = ac.email_cfg.username.clone();
            let resp = email::send(
                ac.email_cfg,
                model::email::Email {
                    from,
                    to: ac.email,
                    subject: format!("激活账号"),
                    body: format!("你的激活码是：{}", ac.code),
                },
            )
            .await
            .unwrap();

            tracing::info!("Send email: {:?}", resp);
            delivery.ack(BasicAckOptions::default()).await.unwrap();
        },
    )
    .await
    .unwrap();
}
