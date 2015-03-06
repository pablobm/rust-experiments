#![feature(io)]
#![feature(fs)]
#![feature(core)]
#![feature(path)]

#![feature(plugin)]
#![plugin(regex_macros)]
extern crate regex;

use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    for target in std::env::args() {
        scan_file(target.as_slice());
    }
}

struct FileLines {
    source: BufReader<File>,
}

impl FileLines {
    fn new(path_str: &str) -> FileLines {
        let path = std::path::Path::new(path_str);
        let file = File::open(&path).unwrap();
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
    let re = regex!(r"[0-9][0-9]\.[0-9]ms\)");
    for line in FileLines::new(path_str) {
        if re.is_match(line.as_slice()) {
            print!("{}", line);
        }
    }
}
