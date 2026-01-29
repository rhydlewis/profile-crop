# ccrop - Circular Crop CLI Tool

A fast, simple command-line tool that downloads images from URLs and applies a circular crop with transparent background.

## Features

- Download images from any HTTP/HTTPS URL
- Automatic center cropping for non-square images
- Circular mask with transparent background
- PNG output with alpha channel
- Supports all common image formats (JPEG, PNG, GIF, WebP, BMP, TIFF)

## Installation

### Download Prebuilt Binaries (Recommended)

Download the latest release for your platform from the [Releases page](https://github.com/rhydlewis/profile-crop/releases):

**macOS:**
```bash
# Intel Macs
curl -L https://github.com/rhydlewis/profile-crop/releases/latest/download/ccrop-macos-x86_64 -o ccrop
chmod +x ccrop
sudo mv ccrop /usr/local/bin/

# Apple Silicon (M1/M2/M3)
curl -L https://github.com/rhydlewis/profile-crop/releases/latest/download/ccrop-macos-aarch64 -o ccrop
chmod +x ccrop
sudo mv ccrop /usr/local/bin/
```

**Linux:**
```bash
# x86_64
curl -L https://github.com/rhydlewis/profile-crop/releases/latest/download/ccrop-linux-x86_64 -o ccrop
chmod +x ccrop
sudo mv ccrop /usr/local/bin/

# ARM64
curl -L https://github.com/rhydlewis/profile-crop/releases/latest/download/ccrop-linux-aarch64 -o ccrop
chmod +x ccrop
sudo mv ccrop /usr/local/bin/
```

**Windows:**
```powershell
# Download ccrop-windows-x86_64.exe from the releases page
# Add the directory containing ccrop.exe to your PATH
```

### Install with Cargo (Rust)

If you have Rust installed:

```bash
cargo install --git https://github.com/rhydlewis/profile-crop
```

## Usage

Basic usage with default output (saves to `output.png`):
```bash
ccrop https://example.com/photo.jpg
```

Specify custom output path:
```bash
ccrop https://example.com/photo.jpg --output avatar.png
ccrop https://example.com/photo.jpg -o ~/Pictures/profile.png
```

View help:
```bash
ccrop --help
```

## Examples

Create a circular profile picture:
```bash
ccrop https://github.com/username.png -o profile.png
```

Process an image and save to specific location:
```bash
ccrop https://example.com/team-photo.jpg -o ~/Desktop/cropped-avatar.png
```

## How It Works

1. **Download**: Fetches the image from the provided URL (30-second timeout)
2. **Center Square**: For non-square images, extracts the center square region
3. **Circular Crop**: Applies a circular mask, making everything outside the circle transparent
4. **Save**: Outputs as PNG with alpha channel for transparency

## Building from Source

Requires Rust 1.70 or later:

```bash
git clone https://github.com/rhydlewis/profile-crop
cd profile-crop
cargo build --release
./target/release/ccrop --help
```

## Error Handling

The tool provides clear error messages for common issues:
- Invalid URLs
- Network failures and timeouts
- Unsupported image formats
- File write permissions

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
