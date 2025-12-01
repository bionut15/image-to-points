use crate::image_process::*;
use clap::Parser;
use std::fs::File;
use std::io::Write;

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


pub fn Cli() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let json_value = convert_image_to_json(&args.inputPath)?;
    let json_string = serde_json::to_string_pretty(&json_value)?;
    
    write_to_json(&json_string, &args.outputFile)?;
    
    println!("{}", json_value);
    
    Ok(())
}
