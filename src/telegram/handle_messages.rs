use futures::StreamExt;
use telegram_bot::Api;

pub async fn handle_messages(api: &Api) {
    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        let update = update.unwrap();

        // Only match on messages that starts with "/download"
        match update.kind {
            telegram_bot::UpdateKind::Message(message) => match message.kind {
                telegram_bot::MessageKind::Text {
                    data: message,
                    entities: _,
                } => {
                    if !message.starts_with("/download") {
                        continue;
                    }

                    println!("{:#?}", message);
                }
                _ => (),
            },
            _ => (),
        }
    }
}
