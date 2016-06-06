extern crate regex;
use regex::Regex;

use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;
use std::process::exit;

fn main() {
    for target in std::env::args() {
        scan_file(target.as_str());
    }
}

struct FileLines {
    source: BufReader<File>,
}

impl FileLines {
    fn new(path_str: &str) -> FileLines {
        let path = std::path::Path::new(path_str);
        let file = match File::open(&path) {
            Ok(f) => f,
            Err(e) => {
                println!("Error opening file: {}", e);
                exit(1);
            }
        };
        let reader = BufReader::new(file);
        FileLines { source: reader }
    }
}

impl Iterator for FileLines {
    type Item = String;

    fn next(&mut self) -> Option<<FileLines as Iterator>::Item> {
        let mut buf = String::new();
        match self.source.read_line(&mut buf) {
            Ok(_) => {
                if buf.is_empty() {
                    None
                }
                else {
                    Some(buf)
                }
            }
            Err(_) => None,
        }
    }
}

fn scan_file(path_str: &str) {
    let re = Regex::new(r"[0-9][0-9]\.[0-9]ms\)").unwrap();
    for line in FileLines::new(path_str) {
        if re.is_match(line.as_str()) {
            print!("{}", line);
        }
    }
}
