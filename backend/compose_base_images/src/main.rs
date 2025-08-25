use std::path::PathBuf;

use clap::Parser;
use compose_base_images::{
    compose_base_images,
    reverse_projection,
    utils::load_base_images,
};

#[derive(Parser)]
#[command(
    about = "Creates composed and deskewed image of Clash of Clans base from multiple screenshots."
)]
struct Cli {
    /// Path to directory with images to compose. Images are split into "left"
    /// and "right" columns and also have index of position in the column.
    /// Example: "l0.jpg" - left top image, "r2.jpg" - third right image,
    /// counted from top to bottom.
    #[arg(short, long)]
    images: PathBuf,
    /// Path to composed image output. Optional.
    #[arg(short, long)]
    composed: Option<PathBuf>,
    /// Path to deskewed image output. Optional.
    #[arg(short, long)]
    reversed: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    let (left, right) = load_base_images(&cli.images).unwrap();

    let composed = compose_base_images(&left, &right);

    if let Some(composed_path) = cli.composed {
        composed.save(composed_path).unwrap();
    }

    let reversed = reverse_projection(&composed);

    if let Some(reversed_path) = cli.reversed {
        reversed.save(reversed_path).unwrap();
    }
}
