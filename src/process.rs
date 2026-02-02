use crate::{CropError, Result};
use image::{DynamicImage, ImageBuffer, Rgba};
use arboard::{Clipboard, ImageData};

pub fn apply_circular_crop(img: DynamicImage) -> Result<DynamicImage> {
    // Convert to RGBA to support transparency
    let img = img.to_rgba8();
    let (width, height) = img.dimensions();

    // Calculate center square dimensions
    let size = width.min(height);
    let x_offset = (width - size) / 2;
    let y_offset = (height - size) / 2;

    // Extract center square
    let square = image::imageops::crop_imm(&img, x_offset, y_offset, size, size).to_image();

    // Create new image buffer for circular crop
    let mut output = ImageBuffer::new(size, size);

    // Calculate circle parameters
    let radius = size as f32 / 2.0;
    let center_x = radius;
    let center_y = radius;

    // Apply circular mask
    for (x, y, pixel) in square.enumerate_pixels() {
        let dx = x as f32 - center_x;
        let dy = y as f32 - center_y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance <= radius {
            // Inside circle - copy pixel
            output.put_pixel(x, y, *pixel);
        } else {
            // Outside circle - make transparent
            output.put_pixel(x, y, Rgba([0, 0, 0, 0]));
        }
    }

    Ok(DynamicImage::ImageRgba8(output))
}

pub fn save_image(img: &DynamicImage, output_path: &str) -> Result<()> {
    img.save(output_path).map_err(|e| {
        CropError::FileWriteError(format!("Failed to save to '{}': {}", output_path, e))
    })
}

pub fn copy_to_clipboard(img: &DynamicImage) -> Result<()> {
    // Convert DynamicImage to RGBA8 format
    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();

    // Create ImageData for clipboard
    let img_data = ImageData {
        width: width as usize,
        height: height as usize,
        bytes: rgba.as_raw().into(),
    };

    // Copy to clipboard
    let mut clipboard = Clipboard::new()
        .map_err(|e| CropError::ClipboardError(e.to_string()))?;

    clipboard
        .set_image(img_data)
        .map_err(|e| CropError::ClipboardError(e.to_string()))?;

    Ok(())
}
