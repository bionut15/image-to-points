use crate::image_process::*;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Output file path for the JSON
    #[arg(short, long)]
    outputFile: String,

    /// Input path for the image
    #[arg(short, long)]
    inputPath: String,

    /// Number of colors for the SVG
    #[arg(short, long, default_value_t = 6)]
    colorCount: u8,
}

pub fn handleinput() {
    let args = Args::parse();
    convert_image_to_json(&args.inputPath);
}
