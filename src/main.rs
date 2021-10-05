use std::env;

use dotenv::dotenv;
use telegram::handle_messages;

mod downloader;
mod telegram;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("TELEGRAM_BOT_API").expect("Missing TELEGRAM_BOT_API env");
    let api = telegram_bot::Api::new(token);

    handle_messages(&api).await;
}
