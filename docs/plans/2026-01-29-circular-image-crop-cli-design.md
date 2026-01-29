# Circular Image Crop CLI Tool - Design Document

**Date:** 2026-01-29
**Status:** Approved
**Command Name:** `ccrop`

## Overview

A Rust CLI tool that downloads an image from a URL, applies a circular crop with transparent background, and saves it as PNG.

## Architecture & Dependencies

**Core Architecture:**
- Rust CLI application with blocking I/O (no async complexity needed)
- Pure Rust dependencies for portability and single-binary distribution

**Dependencies:**
- `clap` (v4) - Argument parsing with derives-based API for type-safe CLI
- `reqwest` (blocking feature) - HTTP downloads with automatic redirect handling
- `image` - Image processing with PNG alpha channel support
- `anyhow` - Ergonomic error handling with context
- `thiserror` - Custom error type definitions

**Project Structure:**
```
profile-crop/
├── Cargo.toml
├── docs/
│   └── plans/
│       └── 2026-01-29-circular-image-crop-cli-design.md
├── src/
│   ├── main.rs      # CLI entry point, argument parsing
│   ├── download.rs  # HTTP fetching logic
│   ├── process.rs   # Image cropping algorithm
│   └── lib.rs       # Shared types, error definitions
```

## CLI Interface

**Command Structure:**
```bash
ccrop <URL> [--output <PATH>]
```

**Arguments:**
- `<URL>`: Required positional argument - HTTP/HTTPS image URL
- `-o, --output <PATH>`: Optional output file path (default: `output.png`)

**Example Usage:**
```bash
# Basic usage with default output
ccrop https://example.com/photo.jpg

# Specify output path
ccrop https://example.com/photo.jpg --output avatar.png

# Custom location
ccrop https://example.com/photo.jpg -o ~/Pictures/profile.png
```

**Validation:**
- URL must be valid HTTP/HTTPS format
- Output path parent directory must exist
- Downloaded content must be valid image format
- Clear error messages for each failure case

**User Feedback:**
- Progress messages: "Downloading...", "Processing...", "Saved to output.png"
- Exit codes: 0 for success, 1 for errors

## Image Processing Logic

**Download Phase:**
1. Use `reqwest::blocking::get()` to fetch image from URL
2. Read HTTP response body into memory as bytes
3. Decode using `image::load_from_memory()` to create `DynamicImage`
4. Handle errors: network failures, invalid URLs, unsupported formats, timeouts

**Crop Algorithm:**

1. **Convert to RGBA format**
   - Ensure image has alpha channel for transparency support

2. **Center square extraction** (for non-square images)
   - Calculate square size: `size = min(width, height)`
   - Calculate center offsets:
     - `x_offset = (width - size) / 2`
     - `y_offset = (height - size) / 2`
   - Extract centered square region

3. **Apply circular mask**
   - Create new RGBA image buffer (same dimensions as square)
   - Calculate circle parameters:
     - `radius = size / 2`
     - `center_x = center_y = radius`
   - For each pixel at (x, y):
     - Calculate distance from center: `dist = sqrt((x - center_x)² + (y - center_y)²)`
     - If `dist <= radius`: copy pixel from cropped image
     - If `dist > radius`: set pixel to fully transparent (R=0, G=0, B=0, A=0)

4. **Save as PNG**
   - Use `image::save()` with PNG encoder
   - PNG format preserves alpha channel transparency

**Quality Considerations:**
- No anti-aliasing on circle edge (simple hard mask initially)
- Preserve original image color space and quality
- Handle very small images gracefully (minimum 1px circle)

## Error Handling & Edge Cases

**Error Types** (using `thiserror`)
- `NetworkError`: Download failures (timeout, DNS, connection refused)
- `InvalidUrlError`: Malformed URL string
- `ImageDecodeError`: Unsupported format or corrupted image data
- `FileWriteError`: Cannot write to output path (permissions, disk full)

**Error Message Format:**
```
Error: Failed to download image
  Caused by: Connection timeout after 30 seconds

Error: Invalid image format
  Supported formats: JPEG, PNG, GIF, WebP, BMP, TIFF

Error: Cannot write to '/protected/output.png'
  Permission denied - check directory permissions
```

**Edge Cases:**
- **Tiny images** (1x1): Apply circle mask, result may be single pixel
- **Very large images**: Process in-memory (acceptable for typical profile photos <10MB)
- **HTTP redirects**: Handled automatically by `reqwest` (follows up to 10)
- **HTTPS certificates**: Validate by default, fail on invalid certificates
- **Output file exists**: Overwrite without warning (standard CLI behavior)
- **Network timeouts**: Default 30-second timeout for HTTP requests
- **User interruption**: Handle Ctrl+C gracefully, cleanup partial files

## Success Criteria

1. Successfully downloads images from valid HTTP/HTTPS URLs
2. Produces perfect circle crops with transparent backgrounds
3. Centers the crop on the original image (extracts center square first)
4. Saves output as valid PNG with alpha channel
5. Provides clear error messages for all failure modes
6. Compiles to single binary with no runtime dependencies
7. Handles common edge cases without crashes

## Future Enhancements (Out of Scope)

- Anti-aliased circle edges for smoother appearance
- Configurable background colors (white, black, custom)
- Support for local file input (not just URLs)
- Batch processing multiple images
- Custom output size/dimensions
- Circle offset/position controls
