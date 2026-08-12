use crate::{CropError, Result};
use arboard::{Clipboard, ImageData};
use image::{DynamicImage, ImageBuffer, Rgba};
use std::path::Path;

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

/// Load an image from a file on disk.
pub fn load_image_from_path(path: &Path) -> Result<DynamicImage> {
    image::open(path).map_err(|e| match e {
        // Surface a friendlier message for the common "file not found" /
        // unreadable case; keep decode errors as decode errors.
        image::ImageError::IoError(io) => {
            CropError::FileReadError(format!("Failed to read '{}': {}", path.display(), io))
        }
        other => CropError::ImageDecodeError(other),
    })
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

#[cfg(test)]
mod tests {
    use super::*;
    use image::{GenericImageView, Rgba};

    /// Build an opaque, solid-colour test image of the given dimensions.
    fn solid(width: u32, height: u32, colour: [u8; 4]) -> DynamicImage {
        let buf = ImageBuffer::from_pixel(width, height, Rgba(colour));
        DynamicImage::ImageRgba8(buf)
    }

    #[test]
    fn crops_to_centred_square_of_min_dimension() {
        // Non-square input: result side == min(width, height).
        let out = apply_circular_crop(solid(100, 60, [255, 0, 0, 255])).unwrap();
        assert_eq!(out.dimensions(), (60, 60));

        let out = apply_circular_crop(solid(40, 90, [0, 255, 0, 255])).unwrap();
        assert_eq!(out.dimensions(), (40, 40));
    }

    #[test]
    fn centre_stays_opaque_corners_become_transparent() {
        let out = apply_circular_crop(solid(80, 80, [10, 20, 30, 255])).unwrap();

        // Centre pixel is inside the circle -> original colour preserved.
        let centre = out.get_pixel(40, 40);
        assert_eq!(centre, Rgba([10, 20, 30, 255]));

        // Every corner lies outside the inscribed circle -> fully transparent.
        for (x, y) in [(0, 0), (79, 0), (0, 79), (79, 79)] {
            assert_eq!(out.get_pixel(x, y), Rgba([0, 0, 0, 0]), "corner ({x},{y})");
        }
    }

    #[test]
    fn square_input_keeps_its_size() {
        let out = apply_circular_crop(solid(50, 50, [1, 2, 3, 255])).unwrap();
        assert_eq!(out.dimensions(), (50, 50));
    }

    #[test]
    fn save_writes_a_decodable_png_with_alpha() {
        let out = apply_circular_crop(solid(64, 64, [200, 100, 50, 255])).unwrap();

        let mut path = std::env::temp_dir();
        path.push("ccrop_test_output.png");
        let path_str = path.to_str().unwrap();

        save_image(&out, path_str).unwrap();

        // Round-trips: dimensions preserved and corners still transparent.
        let reloaded = image::open(&path).unwrap();
        assert_eq!(reloaded.dimensions(), (64, 64));
        assert_eq!(reloaded.get_pixel(0, 0), Rgba([0, 0, 0, 0]));

        let _ = std::fs::remove_file(&path);
    }
}
