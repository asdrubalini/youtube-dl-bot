use std::{path::PathBuf, process::Command};

use rand::Rng;

const RANDOM_PATH: &str = "./downloads/";
const RANDOM_FILENAME_LENGTH: usize = 32;

pub trait Downloader {
    fn new(url: &str) -> Self;
    fn download(self) -> Result<PathBuf, DownloaderError>;
}

#[derive(Debug)]
pub enum DownloaderError {
    ProcessError(Option<i32>),
    FileNotCreated,
}

pub struct Mp3Downloader {
    url: String,
    output_path: PathBuf,
}

impl Downloader for Mp3Downloader {
    fn new(url: &str) -> Self {
        let output_path = generate_random_filename(".mp3");

        Self {
            url: url.to_string(),
            output_path,
        }
    }

    fn download(self) -> Result<PathBuf, DownloaderError> {
        // Start download
        let youtube_dl_status_code = Command::new("youtube-dl")
            .args([
                "--extract-audio",
                "--audio-format",
                "mp3",
                "--output",
                &self.output_path.as_os_str().to_string_lossy(),
                &self.url,
            ])
            .status()
            .map_err(|_| DownloaderError::ProcessError(None))?
            .code();

        // Sanity check process return code and error out in case is wasn't success
        if youtube_dl_status_code.is_none()
            || (youtube_dl_status_code.is_some() && youtube_dl_status_code.unwrap() != 0)
        {
            return Err(DownloaderError::ProcessError(youtube_dl_status_code));
        }

        // Make sure that the output file created by youtube-dl was actually created
        if !self.output_path.exists() {
            return Err(DownloaderError::FileNotCreated);
        }

        Ok(self.output_path)
    }
}

fn generate_random_filename(extension: &str) -> PathBuf {
    let mut generator = rand::thread_rng();

    // Generate 32 chars random filename
    let mut filename: String = (0..RANDOM_FILENAME_LENGTH)
        .into_iter()
        .map(|_| {
            let random_byte: u8 = generator.gen_range(0..=255);
            format!("{:02x}", random_byte)
        })
        .collect();

    // Add extension to the end of the file
    filename.push_str(extension);

    // Build the path
    let mut path = PathBuf::from(RANDOM_PATH);
    path.push(filename);

    println!("Random file generated: {:?}", path);

    path
}
