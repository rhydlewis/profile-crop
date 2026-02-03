use clap::Parser;

mod download;
mod process;

use ccrop::{CropError, Result};

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

fn run(cli: Cli) -> Result<()> {
    // Download image
    println!("Downloading...");
    let img = download::download_image(&cli.url)?;

    // Apply circular crop
    println!("Processing...");
    let cropped = process::apply_circular_crop(img)?;

    // Save to file
    process::save_image(&cropped, &cli.output)?;

    println!("Saved to {}", cli.output);
    Ok(())
}

fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
