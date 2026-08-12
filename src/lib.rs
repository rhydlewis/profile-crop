//! Core library for `ccrop` — circular image cropping with a transparent
//! background.
//!
//! The pure transformation is [`apply_circular_crop`]; everything else is a
//! thin helper for loading (path / URL), saving, and copying to the clipboard.

use thiserror::Error;

mod download;
mod process;

pub use download::download_image;
pub use process::{apply_circular_crop, copy_to_clipboard, load_image_from_path, save_image};

#[derive(Error, Debug)]
pub enum CropError {
    #[error("Failed to download image: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("Invalid URL: {0}")]
    InvalidUrlError(String),

    #[error("Failed to decode image: {0}")]
    ImageDecodeError(#[from] image::ImageError),

    #[error("Failed to read file: {0}")]
    FileReadError(String),

    #[error("Failed to write output file: {0}")]
    FileWriteError(String),

    #[error("Clipboard error: {0}")]
    ClipboardError(String),
}

pub type Result<T> = std::result::Result<T, CropError>;
