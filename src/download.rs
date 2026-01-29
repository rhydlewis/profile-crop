use crate::{CropError, Result};
use image::DynamicImage;
use std::time::Duration;

pub fn download_image(url: &str) -> Result<DynamicImage> {
    // Validate URL format
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err(CropError::InvalidUrlError(
            "URL must start with http:// or https://".to_string(),
        ));
    }

    // Create HTTP client with timeout
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()?;

    // Download the image
    let response = client.get(url).send()?;

    // Check if the response was successful
    if !response.status().is_success() {
        return Err(CropError::NetworkError(
            reqwest::Error::from(response.error_for_status().unwrap_err()),
        ));
    }

    // Read response bytes
    let bytes = response.bytes()?;

    // Decode image from bytes
    let img = image::load_from_memory(&bytes)?;

    Ok(img)
}
