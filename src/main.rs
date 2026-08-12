use std::path::Path;

use clap::Parser;

use ccrop::Result;

#[derive(Parser)]
#[command(name = "ccrop", version, disable_version_flag = true)]
#[command(about = "Apply circular crop to images from URLs or local files", long_about = None)]
struct Cli {
    /// URL or path of the image to crop
    source: String,

    /// Output file path (defaults to the source filename with a .png extension)
    #[arg(short, long)]
    output: Option<String>,

    /// Skip copying to clipboard
    #[arg(long, default_value_t = false)]
    no_clipboard: bool,

    /// Print version
    #[arg(short = 'v', long = "version", action = clap::ArgAction::Version)]
    version: Option<bool>,
}

fn is_url(source: &str) -> bool {
    source.starts_with("http://") || source.starts_with("https://")
}

/// Default output filename for a source URL or file path: the source's file
/// stem with a `.png` extension (output is always PNG so transparency
/// survives). Falls back to `output.png` when the source has no usable
/// filename, e.g. a bare domain or a URL ending in `/`.
fn derive_output_name(source: &str) -> String {
    let filename = if is_url(source) {
        // Drop fragment and query, then take the last path segment (if the
        // URL has a path at all).
        let trimmed = source.split(['#', '?']).next().unwrap_or(source);
        match trimmed.split_once("://").map(|(_, rest)| rest) {
            Some(rest) if rest.contains('/') => rest.rsplit('/').next().unwrap_or(""),
            _ => "",
        }
    } else {
        Path::new(source)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
    };

    match Path::new(filename).file_stem().and_then(|s| s.to_str()) {
        Some(stem) if !stem.is_empty() => format!("{stem}.png"),
        _ => "output.png".to_string(),
    }
}

/// True if `output` already exists and is the same file as `source`.
fn would_overwrite_source(source: &str, output: &str) -> bool {
    match (
        std::fs::canonicalize(source),
        std::fs::canonicalize(output),
    ) {
        (Ok(a), Ok(b)) => a == b,
        _ => false,
    }
}

fn run(cli: Cli) -> Result<()> {
    let img = if is_url(&cli.source) {
        println!("Downloading...");
        ccrop::download_image(&cli.source)?
    } else {
        println!("Loading...");
        ccrop::load_image_from_path(Path::new(&cli.source))?
    };

    let output = match cli.output {
        Some(output) => output,
        None => {
            let mut output = derive_output_name(&cli.source);
            // Don't clobber a local input with the derived default; an
            // explicit --output is respected as given.
            if would_overwrite_source(&cli.source, &output) {
                output = format!(
                    "{}-crop.png",
                    output.strip_suffix(".png").unwrap_or(&output)
                );
            }
            output
        }
    };

    // Apply circular crop
    println!("Processing...");
    let cropped = ccrop::apply_circular_crop(img)?;

    // Copy to clipboard (unless disabled)
    if !cli.no_clipboard {
        if let Err(e) = ccrop::copy_to_clipboard(&cropped) {
            eprintln!("Warning: Failed to copy to clipboard: {}", e);
        } else {
            println!("Copied to clipboard");
        }
    }

    // Save to file
    ccrop::save_image(&cropped, &output)?;

    println!("Saved to {}", output);
    Ok(())
}

fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::derive_output_name;

    #[test]
    fn derives_name_from_url() {
        assert_eq!(
            derive_output_name("https://example.com/photo.jpg"),
            "photo.png"
        );
        assert_eq!(
            derive_output_name("https://github.com/username.png"),
            "username.png"
        );
        assert_eq!(
            derive_output_name("https://example.com/a/b/pic.webp?size=200#frag"),
            "pic.png"
        );
    }

    #[test]
    fn derives_name_from_path() {
        assert_eq!(derive_output_name("photo.jpg"), "photo.png");
        assert_eq!(derive_output_name("/tmp/dir/avatar.jpeg"), "avatar.png");
        assert_eq!(derive_output_name("./relative/team photo.tiff"), "team photo.png");
    }

    #[test]
    fn falls_back_when_no_filename() {
        assert_eq!(derive_output_name("https://example.com"), "output.png");
        assert_eq!(derive_output_name("https://example.com/"), "output.png");
        assert_eq!(derive_output_name("https://example.com/dir/"), "output.png");
    }
}
