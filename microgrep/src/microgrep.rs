#![feature(phase)]
#[phase(plugin)]
extern crate regex_macros;
extern crate regex;

use std::os;
use std::io::BufferedReader;
use std::io::File;

fn main() {
    for target in os::args().iter() {
        scan_file(target);
    }
}

fn scan_file(path_str: &String) {
    let re = regex!(r"\([0-9][0-9]\.[0-9]ms\)");
    let path = Path::new(path_str.as_bytes());
    let file = File::open(&path);
    let mut reader = BufferedReader::new(file);

    for line in reader.lines() {
        let s;
        match line {
            Ok(_s) => s = _s,
            Err(_) => return,
        }
        if re.is_match(s.as_slice()) {
            print!("{}", s);
        }
    }
}
