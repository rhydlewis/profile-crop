# Clipboard Support Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add automatic clipboard copying of cropped images with opt-out flag

**Architecture:** Add arboard dependency for cross-platform clipboard support. Extend CLI with --no-clipboard flag. Add clipboard copy function to process.rs. Update main.rs flow to copy before saving (warn on failure, don't block).

**Tech Stack:** arboard 3.4 for clipboard, existing image/clap/reqwest stack

---

### Task 1: Add Clipboard Dependency

**Files:**
- Modify: `Cargo.toml:21`

**Step 1: Add arboard dependency**

Add to dependencies section in Cargo.toml:

```toml
arboard = "3.4"
```

Full dependencies section should be:
```toml
[dependencies]
clap = { version = "4.5", features = ["derive"] }
reqwest = { version = "0.12", features = ["blocking", "rustls-tls"], default-features = false }
image = "0.24"
anyhow = "1.0"
thiserror = "1.0"
arboard = "3.4"
```

**Step 2: Verify dependency resolves**

Run: `cargo check`
Expected: Dependency downloads and compiles successfully

**Step 3: Commit**

```bash
git add Cargo.toml Cargo.lock
git commit -m "feat: add arboard clipboard dependency"
```

---

### Task 2: Add Clipboard Error Type

**Files:**
- Modify: `src/lib.rs:1-18`

**Step 1: Add ClipboardError variant**

Add new error variant to CropError enum:

```rust
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

    #[error("Clipboard error: {0}")]
    ClipboardError(String),
}

pub type Result<T> = std::result::Result<T, CropError>;
```

**Step 2: Verify it compiles**

Run: `cargo check`
Expected: Compiles successfully

**Step 3: Commit**

```bash
git add src/lib.rs
git commit -m "feat: add ClipboardError variant"
```

---

### Task 3: Implement Clipboard Copy Function

**Files:**
- Modify: `src/process.rs:1-48`

**Step 1: Add clipboard import and function**

Add to top of file after existing imports:

```rust
use arboard::{Clipboard, ImageData};
```

Add new function after `save_image`:

```rust
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
```

**Step 2: Verify it compiles**

Run: `cargo check`
Expected: Compiles successfully

**Step 3: Commit**

```bash
git add src/process.rs
git commit -m "feat: implement clipboard copy function"
```

---

### Task 4: Add CLI Flag

**Files:**
- Modify: `src/main.rs:8-18`

**Step 1: Add no_clipboard field to Cli struct**

Update Cli struct:

```rust
#[derive(Parser)]
#[command(name = "ccrop")]
#[command(about = "Apply circular crop to images from URLs", long_about = None)]
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

**Step 2: Verify flag works**

Run: `cargo build`
Expected: Builds successfully

Run: `cargo run -- --help`
Expected: Help text shows `--no-clipboard` option

**Step 3: Commit**

```bash
git add src/main.rs
git commit -m "feat: add --no-clipboard CLI flag"
```

---

### Task 5: Integrate Clipboard in Main Flow

**Files:**
- Modify: `src/main.rs:20-34`

**Step 1: Add clipboard copy logic before file save**

Update `run` function:

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

    // Save to file
    process::save_image(&cropped, &cli.output)?;

    println!("Saved to {}", cli.output);
    Ok(())
}
```

**Step 2: Test default behavior (clipboard enabled)**

Run: `cargo build`
Run: `./target/debug/ccrop https://avatars.githubusercontent.com/u/1?v=4`
Expected:
- Prints "Downloading..."
- Prints "Processing..."
- Prints "Copied to clipboard" (or warning if clipboard unavailable)
- Prints "Saved to output.png"
- Image should be in clipboard (paste into image editor to verify)
- output.png should exist

**Step 3: Test --no-clipboard flag**

Run: `./target/debug/ccrop https://avatars.githubusercontent.com/u/1?v=4 --no-clipboard -o test.png`
Expected:
- Prints "Downloading..."
- Prints "Processing..."
- Does NOT print "Copied to clipboard"
- Prints "Saved to test.png"
- test.png should exist
- Clipboard should NOT be modified

**Step 4: Clean up test files**

Run: `rm output.png test.png`

**Step 5: Commit**

```bash
git add src/main.rs
git commit -m "feat: integrate clipboard copy in main flow"
```

---

### Task 6: Update Documentation

**Files:**
- Modify: `README.md:1-116`

**Step 1: Update Features section**

Update features list (around line 5-11):

```markdown
## Features

- Download images from any HTTP/HTTPS URL
- Automatic center cropping for non-square images
- Circular mask with transparent background
- **Automatic clipboard copy** (with opt-out flag)
- PNG output with alpha channel
- Supports all common image formats (JPEG, PNG, GIF, WebP, BMP, TIFF)
- Cross-platform clipboard support (macOS, Windows, Linux)
```

**Step 2: Update Usage section**

Update usage examples (around line 53-69):

```markdown
## Usage

Basic usage with default output (saves to `output.png` and copies to clipboard):
```bash
ccrop https://example.com/photo.jpg
```

Specify custom output path (still copies to clipboard):
```bash
ccrop https://example.com/photo.jpg --output avatar.png
ccrop https://example.com/photo.jpg -o ~/Pictures/profile.png
```

Skip clipboard copy (file only):
```bash
ccrop https://example.com/photo.jpg --no-clipboard
```

View help:
```bash
ccrop --help
```
```

**Step 3: Update How It Works section**

Update section (around line 84-88):

```markdown
## How It Works

1. **Download**: Fetches the image from the provided URL (30-second timeout)
2. **Center Square**: For non-square images, extracts the center square region
3. **Circular Crop**: Applies a circular mask, making everything outside the circle transparent
4. **Copy to Clipboard**: Automatically copies the result to system clipboard (unless `--no-clipboard` is used)
5. **Save**: Outputs as PNG with alpha channel for transparency
```

**Step 4: Verify documentation looks correct**

Run: `cat README.md | grep -A 3 "## Features"`
Expected: Shows updated features list with clipboard mention

**Step 5: Commit**

```bash
git add README.md
git commit -m "docs: update README with clipboard feature"
```

---

### Task 7: Build and Test Release Binary

**Files:**
- N/A (testing only)

**Step 1: Build release binary**

Run: `cargo build --release`
Expected: Builds successfully

**Step 2: Test release binary with real URL**

Run: `./target/release/ccrop https://avatars.githubusercontent.com/u/1?v=4 -o final-test.png`
Expected:
- Downloads and processes successfully
- Prints "Copied to clipboard"
- Creates final-test.png
- Image in clipboard can be pasted

**Step 3: Test --no-clipboard flag**

Run: `./target/release/ccrop https://avatars.githubusercontent.com/u/1?v=4 -o no-clip-test.png --no-clipboard`
Expected:
- Downloads and processes successfully
- Does NOT print "Copied to clipboard"
- Creates no-clip-test.png
- Clipboard unchanged

**Step 4: Clean up test files**

Run: `rm final-test.png no-clip-test.png`

**Step 5: Commit if any fixes were needed**

If no fixes needed, skip this step.
Otherwise:
```bash
git add [modified files]
git commit -m "fix: [description of fix]"
```

---

## Testing Checklist

- [ ] `cargo check` passes
- [ ] `cargo build` passes
- [ ] `cargo build --release` passes
- [ ] Default behavior copies to clipboard
- [ ] `--no-clipboard` flag skips clipboard
- [ ] File is saved even if clipboard fails
- [ ] Warning printed if clipboard fails
- [ ] Help text shows `--no-clipboard` option
- [ ] README accurately describes new feature

## Notes for Implementation

- Clipboard copy happens BEFORE file save to fail fast if clipboard issues
- Clipboard failures are non-fatal - they print a warning and continue
- The `arboard` crate handles all platform differences internally
- No conditional compilation needed - works on macOS, Windows, Linux
- Image is converted to RGBA8 format for clipboard compatibility
- Transparency is preserved in clipboard data
