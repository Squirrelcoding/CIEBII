mod convert;
mod render;
mod icons;

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
    Render {
        file_name: String,
    },

    /// Converts a PNG/JPG file into a ciebii file
    Convert {
        i: String,
        o: String,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Args::parse();

    match &cli.command {
        Commands::Render { file_name } => {
            render(file_name.to_owned())?;
        }
        Commands::Convert { i, o } => match convert::convert(i, o) {
            Ok(_) => {
                println!(
                    "✨ {} {} {} {}{}",
                    "Successfully converted".green().bold(),
                    format!("'{}'", i).white().bold(),
                    "to".green().bold(),
                    format!("'{}'", o).white().bold(),
                    "!".green().bold(),
                );
            }
            Err(err) => {
                println!(
                    "{} {} {} {}{}",
                    "Failed to convert".red().bold(),
                    format!("'{}'", i).white().bold(),
                    "to".red().bold(),
                    format!("'{}'", o).white().bold(),
                    ".".red().bold(),
                );

                eprintln!("{err}");

                std::fs::remove_file(o)?;
            }
        },
    }

    Ok(())
}
