#![feature(phase)]
#[phase(plugin)]
extern crate regex_macros;
extern crate regex;

use std::os;
use std::io::{BufferedReader, File, Lines};

fn main() {
    for target in os::args().iter() {
        scan_file(target.as_slice());
    }
}

struct FileLines {
    source: BufferedReader<File>
}

impl FileLines {
    fn new(path_str: &str) -> FileLines {
        let path = Path::new(path_str.as_bytes());
        let file = File::open(&path).unwrap();
        let reader = BufferedReader::new(file);
        FileLines { source: reader }
    }

    fn lines(&mut self) -> Lines<BufferedReader<File>> {
        self.source.lines()
    }
}

fn scan_file(path_str: &str) {
    let re = regex!(r"[0-9][0-9]\.[0-9]ms\)");
    for line in FileLines::new(path_str).lines() {
        match line {
            Ok(s) => {
                if re.is_match(s.as_slice()) {
                    print!("{}", s);
                }
            }
            Err(_) => return
        }
    }
}
