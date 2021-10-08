use std::{
    fs::remove_file,
    path::PathBuf,
    process::{Command, Stdio},
};

use rand::Rng;

const RANDOM_PATH: &str = "./downloads/";
const RANDOM_FILENAME_LENGTH: usize = 32;

pub trait Downloader {
    fn new(url: &str) -> Self;
    fn download(&self) -> Result<PathBuf, DownloaderError>;
    fn cleanup(self);
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
    /// Create a Downloader instance
    fn new(url: &str) -> Self {
        let output_path = generate_random_filename(".mp3");

        Self {
            url: url.to_string(),
            output_path,
        }
    }

    /// Start download process
    fn download(&self) -> Result<PathBuf, DownloaderError> {
        // Start download, suppress output and capture code
        log::trace!("Starting youtube-dl process");

        let download_status = Command::new("youtube-dl")
            .args([
                "--extract-audio",
                "--audio-format",
                "mp3",
                "--output",
                &self.output_path.as_os_str().to_string_lossy(),
                &self.url,
            ])
            .stdout(Stdio::null())
            .status()
            .map_err(|_| DownloaderError::ProcessError(None))?
            .code();

        // Sanity check process return code and error out in case is wasn't success
        if download_status.is_none() || (download_status.is_some() && download_status.unwrap() != 0)
        {
            log::error!(
                "youtube-dl returned non-zero status code for {:?}",
                &self.output_path
            );
            return Err(DownloaderError::ProcessError(download_status));
        }

        // Make sure that the output file created by youtube-dl was actually created
        if !self.output_path.exists() {
            log::error!(
                "youtube-dl didn't create mp3 file for {:?}",
                &self.output_path
            );
            return Err(DownloaderError::FileNotCreated);
        }

        Ok(self.output_path.clone())
    }

    /// Remove generated file
    fn cleanup(self) {
        match remove_file(self.output_path.clone()) {
            Ok(_) => log::trace!("Removed {:?}", self.output_path),
            Err(error) => log::trace!("Error while removing {:?}: {}", self.output_path, error),
        }
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

    log::trace!("Random file generated: {:?}", path);

    path
}
