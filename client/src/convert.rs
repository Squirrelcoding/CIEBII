use std::{fs::File, path::Path};

use anyhow::Context;
use ciebii_lib::{chunk::Chunk, file::CIEBIIFILE, io::write_file};
use colored::*;
use image::GenericImageView;

pub fn convert(i: &str) -> anyhow::Result<()> {
    let input_path = Path::new(i);

    let out_path = format!(
        "{}.cib",
        input_path.file_stem().unwrap().to_str().unwrap()
    );
    
    let o = Path::new(&out_path);

    File::create(o)?;

    let image = image::open(i).with_context(|| {
        format!(
            "{} {}{}",
            "Failed to open".red().bold(),
            format!("'{}'", i).white().bold(),
            "!".red().bold()
        )
    })?;

    let width = image.width() as usize;
    let height = image.height() as usize;

    let chunks = image
        .pixels()
        .into_iter()
        .map(|pixel| Chunk::new(pixel.2[0], pixel.2[1], pixel.2[2]))
        .collect();
    println!("🌈 {}", "Converting colors...".bold());

    println!("⚒️ {}", "constructing file...".bold());
    let ciebii_file = CIEBIIFILE::try_from_chunks(width, height, chunks)?;

    write_file(Path::new(o), &ciebii_file)?;
    println!("💾 {}", "saving file...".bold());

    Ok(())
}
