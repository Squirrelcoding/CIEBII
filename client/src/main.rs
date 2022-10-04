mod convert;
mod icons;
mod render;

use std::path::Path;

use clap::{Parser, Subcommand};
use colored::*;
use macroquad::prelude::*;
use render::render;

/// ✨ Ciebii file viewer ✨
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Renders a ciebii file
    Render { file_name: String },

    /// Converts a PNG/JPG file into a ciebii file
    Convert { i: String },
}

fn main() -> anyhow::Result<()> {
    let cli = Args::parse();

    match &cli.command {
        Commands::Render { file_name } => {
            render(file_name.to_owned())?;
        }
        Commands::Convert { i } => match convert::convert(i) {
            Ok(_) => {
                println!(
                    "✨ {} {}{}",
                    "Successfully converted".green().bold(),
                    format!("'{}'", i).white().bold(),
                    "!".green().bold()
                );
            }
            Err(err) => {
                println!(
                    "{} {}{}",
                    "Failed to convert".red().bold(),
                    format!("'{}'", i).white().bold(),
                    ".".red().bold()
                );

                eprintln!("{err}");

                std::fs::remove_file(Path::new(i).file_stem().unwrap().to_str().unwrap())?;
            }
        },
    }

    Ok(())
}
