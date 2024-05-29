mod stitcher;
mod utils;

use clap::Parser;
use std::fs;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    root: String,

    #[arg(short, long)]
    output: String,

    #[arg(short, long)]
    directive: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let root_file = cli.root.as_str();
    let stitch_pattern = cli.directive.unwrap_or("@mdstitch".to_string());

    let index = utils::read_from_file(root_file);
    if index.is_none() {
        return;
    }
    let index = index.unwrap();

    let result = stitcher::stitch(stitch_pattern.as_str(), index.as_str());
    if result.is_none() {
        return;
    }
    let result = result.unwrap();

    let output_file = cli.output.as_str();
    match fs::write(output_file, result) {
        Err(error) => eprintln!("Could not save output to file {}: {}", output_file, error),
        Ok(_) => println!("Saved to {}", cli.output)
    }
}