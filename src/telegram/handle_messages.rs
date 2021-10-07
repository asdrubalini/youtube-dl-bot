use futures::StreamExt;
use telegram_bot::Api;

use crate::telegram::download_tasks::mp3_download_task;

pub async fn handle_messages(api: &Api) {
    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        let update = update.unwrap();

        // Only match messages that starts with certain commands
        match update.kind {
            telegram_bot::UpdateKind::Message(message) => match message.kind {
                telegram_bot::MessageKind::Text {
                    data: message_text,
                    entities: _,
                } => {
                    if message_text.starts_with("/audio") {
                        let youtube_url = message_text.replace("/audio ", "");
                        let chat_id = message.chat.id();
                        let api = api.clone();

                        tokio::task::spawn(async move {
                            mp3_download_task(api, youtube_url, chat_id).await
                        });
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }
}
