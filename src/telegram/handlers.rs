use futures::StreamExt;
use telegram_bot::{Api, ChatId};

use crate::telegram::download_tasks::mp3_download_task;

/// Handle all events
pub async fn handle_events(api: &Api) {
    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        let update = update.unwrap();

        // Only match text messages
        match update.kind {
            telegram_bot::UpdateKind::Message(message) => match message.kind {
                telegram_bot::MessageKind::Text { data, entities: _ } => {
                    let chat_id = message.chat.id();
                    handle_text_message(api, data, chat_id).await;
                }
                _ => (),
            },
            _ => (),
        }
    }
}

/// Text message handler
pub async fn handle_text_message(api: &Api, message_text: String, chat_id: ChatId) {
    // Audio download command
    if message_text.starts_with("/audio") {
        let youtube_url = message_text.replace("/audio ", "");
        let api = api.clone();

        tokio::task::spawn(async move { mp3_download_task(api, youtube_url, chat_id).await });
    }
}
