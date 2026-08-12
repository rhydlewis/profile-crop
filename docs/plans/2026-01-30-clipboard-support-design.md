# Clipboard Support Design

## Overview

Add clipboard functionality to ccrop so that the circular-cropped image is automatically copied to the system clipboard by default, while still saving to a file.

## Requirements

- Copy cropped image to clipboard by default
- Allow users to opt out with `--no-clipboard` flag
- Warn and continue if clipboard fails (don't block file saving)
- Support all platforms (macOS, Windows, Linux)

## CLI Interface Changes

Update the command-line interface to include a new flag:

```rust
#[derive(Parser)]
struct Cli {
    /// URL of the image to download and crop
    url: String,

    /// Output file path (defaults to output.png)
    #[arg(short, long, default_value = "output.png")]
    output: String,

    /// Skip copying to clipboard
    #[arg(long, default_value_t = false)]
    no_clipboard: bool,
}
```

### Behavior

- By default (`no_clipboard = false`): Tool saves file AND copies to clipboard
- With `--no-clipboard` flag: Tool only saves file, skips clipboard
- The file is always saved - clipboard is an additional step

### Example Usage

```bash
ccrop https://example.com/photo.jpg              # Saves + copies to clipboard
ccrop https://example.com/photo.jpg --no-clipboard  # Only saves file
```

## Dependencies

Add the `arboard` crate for cross-platform clipboard support:

```toml
[dependencies]
arboard = "3.4"  # Cross-platform clipboard support
```

The `arboard` crate handles all platform-specific clipboard operations internally, providing support for macOS, Windows, and Linux without requiring conditional compilation.

## Implementation Flow

Update the main execution flow in `main.rs`:

```rust
fn run(cli: Cli) -> Result<()> {
    // Download image
    println!("Downloading...");
    let img = download::download_image(&cli.url)?;

    // Apply circular crop
    println!("Processing...");
    let cropped = process::apply_circular_crop(img)?;

    // Copy to clipboard (unless disabled)
    if !cli.no_clipboard {
        if let Err(e) = process::copy_to_clipboard(&cropped) {
            eprintln!("Warning: Failed to copy to clipboard: {}", e);
            // Continue execution - file will still be saved
        } else {
            println!("Copied to clipboard");
        }
    }

    // Save to file (always happens)
    process::save_image(&cropped, &cli.output)?;
    println!("Saved to {}", cli.output);

    Ok(())
}
```

### Key Points

- Clipboard operation happens before file save
- Uses `if let Err` to catch clipboard failures and print warning
- Execution continues to file save even if clipboard fails
- Success message printed only if clipboard copy succeeds
- File save is unaffected by clipboard status

## Clipboard Implementation

Add new function to `src/process.rs`:

```rust
use arboard::{Clipboard, ImageData};

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

    clipboard.set_image(img_data)
        .map_err(|e| CropError::ClipboardError(e.to_string()))?;

    Ok(())
}
```

### Image Format

- Converts to RGBA8 (standard format with alpha channel)
- Preserves transparency from circular crop
- Compatible with most clipboard-aware applications

## Error Handling

Add new error variant to `src/lib.rs`:

```rust
#[derive(Debug, thiserror::Error)]
pub enum CropError {
    // ... existing variants ...

    #[error("Clipboard error: {0}")]
    ClipboardError(String),
}
```

### Error Behavior

- Clipboard failures print a warning message
- Execution continues to save the file
- Command exits with success code 0 (clipboard is not critical)
- Other platforms supported by arboard should work without additional code

## Documentation Updates

Update README.md to document:
- New default clipboard behavior
- The `--no-clipboard` flag
- Platform compatibility notes
- Updated usage examples
