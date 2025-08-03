use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Error, ErrorKind, Lines, Write},
    ops::Add,
    path::{Path, PathBuf},
    str::SplitWhitespace,
};

use anyhow::anyhow;
use build::{
    color_change::{ColorChange, parse_color_change},
    color_map::COLORS,
    unicode_insert::UnicodeInsert,
};

mod build {
    pub(super) mod color_change;
    pub(super) mod color_map;
    pub(super) mod unicode_insert;
}

/// Main entry point for the build script.
fn main() -> anyhow::Result<()> {
    // Link CoreGraphics for macOS
    if std::env::var("CARGO_CFG_TARGET_OS")? == "macos" {
        println!("cargo:rustc-link-lib=framework=CoreGraphics");
    }

    // Preprocess distro logos
    let in_dir: &Path = Path::new("static/logos");
    let out_dir: &Path = Path::new("static/logos/sh");
    fs::create_dir_all(out_dir).expect("Failed to create output directory");

    for entry in fs::read_dir(in_dir)? {
        // Some Initial setup stuff
        let path: PathBuf = entry?.path();

        // Skip non-files
        if !path.is_file() {
            continue;
        }

        let out_path: PathBuf = out_dir.join(
            path.file_name()
                .ok_or(anyhow!("Failed to read file name"))?,
        );
        let mut content: Lines<BufReader<File>> = BufReader::new(File::open(&path)?).lines();

        // Parse first line
        let first_line: String = content
            .next()
            .ok_or_else(|| Error::new(ErrorKind::UnexpectedEof, "File is empty"))??;
        let mut logo_metadata: SplitWhitespace<'_> = first_line.split_whitespace();
        let rows: u16 = logo_metadata
            .next()
            .ok_or(anyhow!(
                "Failed to read row count: {}",
                path.to_string_lossy()
            ))?
            .parse::<u16>()?;
        let cols: u16 = logo_metadata
            .next()
            .ok_or(anyhow!(
                "Failed to read col count: {}",
                path.to_string_lossy()
            ))?
            .parse::<u16>()?;

        // Read logo and pad to length
        let mut logo: Vec<String> = Vec::with_capacity(rows as usize);
        for _ in 0..rows {
            let row_content: String = content
                .next()
                .ok_or(anyhow!("Failed to read: {}", path.to_string_lossy()))??;
            logo.push(format!("{:<width$}", row_content, width = cols as usize));
        }

        // Insert color codes
        let color_changes: Vec<ColorChange> = content
            .map(|line: Result<String, Error>| parse_color_change(&line.unwrap()))
            .collect();
        for change in color_changes.iter().rev() {
            logo[change.row as usize].insert_str_unicode(change.col as usize, &change.bash_code);
        }

        // Insert color codes in the beginning and end
        // Build prefix sum array
        let mut psa: Vec<String> = vec![String::new(); rows as usize];
        for change in color_changes.iter() {
            psa[change.row as usize] = change.bash_code.clone();
        }

        // Insert color codes
        let mut prev_line_color: &String = &psa[0];
        logo[0] += COLORS["reset"];
        for i in 1..logo.len() {
            logo[i].insert_str_unicode(0, prev_line_color);
            logo[i] += COLORS["reset"];

            if !psa[i].is_empty() {
                prev_line_color = &psa[i];
            }
        }

        // Write to output
        let mut out_file: File = File::create(&out_path)?;
        out_file.write_all(first_line.add("\n").as_bytes())?;
        out_file.write_all(logo.join("\n").as_bytes())?;
    }

    Ok(())
}
