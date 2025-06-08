use crate::build::color_map::COLORS;

pub struct ColorChange {
    pub row: u16,
    pub col: u16,
    pub bash_code: String,
}

pub fn parse_color_change(color_change: &str) -> ColorChange {

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