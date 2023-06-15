use axum_rabbitmq_lettre::Config;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt().init();

    let _cfg = Config::from_env().unwrap();
}
