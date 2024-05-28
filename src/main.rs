use clap::{Parser};
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

fn read_from_file(path: &str) -> Option<String> {
    return match fs::read_to_string(path) {
        Err(error) => {
            eprintln!("Could not read '{}': {}", path, error);
            return None;
        }
        Ok(content) => Some(content)
    };
}

fn merge(stitch_pattern: &str, index: &str) -> Option<String> {
    let mut merged = String::new();
    for line in index.lines() {
        let line = line.trim_start();
        if line.len() > stitch_pattern.len() + 3 && line.trim_start().starts_with(&stitch_pattern) {
            let (_, path) = line.split_at(stitch_pattern.len());
            let path = &path[1..path.len() - 1];

            let content = read_from_file(path);
            if content.is_none() {
                return None;
            }

            let content = merge(stitch_pattern, content.unwrap().as_str());
            if content.is_none() {
                return None;
            }
            let content = content.unwrap();

            merged.push_str(content.as_str());
        } else {
            merged.push_str(line);
            merged.push('\n');
        }
    }
    Some(merged)
}

fn main() {
    let cli = Cli::parse();

    let root_file = cli.root.as_str();
    let stitch_pattern = cli.directive.unwrap_or("@mdstitch".to_string());

    let index = read_from_file(root_file);
    if index.is_none() {
        return;
    }
    let index = index.unwrap();

    let result = merge(stitch_pattern.as_str(), index.as_str());
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