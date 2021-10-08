use std::env;

use dotenv::dotenv;
use telegram::handle_events;

mod downloader;
mod telegram;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("TELEGRAM_BOT_API").expect("Missing TELEGRAM_BOT_API env");
    let api = telegram_bot::Api::new(token);

    log::info!("Starting bot...");

    handle_events(&api).await;
}
