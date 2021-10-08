use telegram_bot::{Api, ChatId, ChatRef, InputFileUpload, SendAudio, SendMessage};

use crate::downloader::{Downloader, Mp3Downloader};

pub async fn mp3_download_task(api: Api, youtube_url: String, chat_id: ChatId) {
    let downloader = Mp3Downloader::new(&youtube_url);
    let chat_ref = ChatRef::from_chat_id(chat_id);

    let audio_path = match downloader.download() {
        Ok(path) => path,
        Err(err) => {
            println!("Got error: {:?}", err);

            api.send(SendMessage::new(chat_ref, "Cannot start download"))
                .await
                .unwrap();

            return;
        }
    };

    // TODO: exception handling

    let audio_file =
        InputFileUpload::with_path(audio_path.as_os_str().to_string_lossy().to_string());

    api.send(SendAudio::new(chat_ref, audio_file))
        .await
        .unwrap();

    downloader.cleanup();
}
