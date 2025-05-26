use anyhow::anyhow;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io;
use std::io::{BufRead, Write};
use std::path::Path;
use std::sync::LazyLock;

static COLORS: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    HashMap::from([
        ("red", "\x1b[0;31m"),
        ("green", "\x1b[0;32m"),
        ("yellow", "\x1b[0;33m"),
        ("blue", "\x1b[0;34m"),
        ("magenta", "\x1b[0;35m"),
        ("cyan", "\x1b[0;36m"),
        ("white", "\x1b[0;37m"),
        ("bold", "\x1b[1m"),
        ("soft", "\x1b[2m"),
        ("reset", "\x1b[0m"),
    ])
});


struct ColorChange {
    row: u16,
    col: u16,
    bash_code: String,
}

fn parse_color_change(color_change: &str) -> ColorChange {

    let mut iter = color_change.split_whitespace();

    let row = iter.next().unwrap().parse::<u16>().unwrap();
    let col = iter.next().unwrap().parse::<u16>().unwrap();

    let names: Vec<&str> = iter.collect();
    let mut codes: Vec<&str> = Vec::with_capacity(names.len());

    for name in names {
        codes.push(COLORS[name])
    }

    ColorChange {
        row,
        col,
        bash_code: codes.concat(),
    }
}


fn main() -> anyhow::Result<()> {

    // === 2. DIR SETUP ===
    let in_dir = Path::new("static/logos");
    let out_dir = Path::new("static/logos/sh");
    fs::create_dir_all(out_dir).expect("Failed to create output directory");

    // === 3. FILE PROCESSING ===
    for entry in fs::read_dir(in_dir)? {

        // Some Initial setup stuff
        let path = entry?.path();

        // Skip non-files
        if ! path.is_file() {
            continue;
        }

        let out_path = out_dir.join(path.file_name().ok_or(anyhow!("Failed to read file name"))?);
        let mut content = io::BufReader::new(File::open(&path)?).lines();

        // Parse first line
        let first_line = content.next().ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "File is empty"))??;
        let mut logo_metadata = first_line.split_whitespace();
        let rows = logo_metadata.next().ok_or(anyhow!("Failed to read row count: {}", path.to_string_lossy()))?.parse::<u16>()?;
        let cols = logo_metadata.next().ok_or(anyhow!("Failed to read col count: {}", path.to_string_lossy()))?.parse::<u16>()?;


        // Read logo and pad to length
        let mut logo: Vec<String> = Vec::with_capacity(rows as usize);
        for _ in 0..rows {
            let row_content = content.next().ok_or(anyhow!("Failed to read: {}", path.to_string_lossy()))??;
            logo.push(format!("{:<width$}", row_content, width = cols as usize));
        }

        // Insert color codes
        let color_changes: Vec<ColorChange> = content.map(|line| parse_color_change(&line.unwrap())).collect();
        for change in color_changes.iter().rev() {
            logo[change.row as usize].insert_str(change.col as usize, &change.bash_code);
        }

        // Write to output
        File::create(&out_path)?.write_all(logo.join("\n").as_bytes())?;

    }

    Ok(())
}
