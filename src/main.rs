use std::fs::File;
use std::io::{BufRead, BufReader};

const LEFT_BRACES: &str = "{";
const RIGHT_BRACES: &str = "}";
const SEMICOLON: &str = ";";

fn read_file_lines(filename: &str) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        lines.push(line?)
    }

    Ok(lines)
}

fn linter(lines: &mut Vec<String>) {
    for line in lines.iter_mut() {
        if line.is_empty() ||
            line.ends_with(LEFT_BRACES) ||
            line.ends_with(RIGHT_BRACES) ||
            line.ends_with(SEMICOLON) {
            continue;
        }

        *line += ";"
    }
}

fn main() {
    let mut lines = read_file_lines("./main.a").unwrap();
    linter(&mut lines);
    println!("{:?}", lines)
}
