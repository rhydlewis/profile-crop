use thiserror::Error;

#[derive(Error, Debug)]
pub enum CropError {
    #[error("Failed to download image: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("Invalid URL: {0}")]
    InvalidUrlError(String),

    #[error("Failed to decode image: {0}")]
    ImageDecodeError(#[from] image::ImageError),

    #[error("Failed to write output file: {0}")]
    FileWriteError(String),
}

pub type Result<T> = std::result::Result<T, CropError>;
