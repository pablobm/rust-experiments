use std::env::args;
use std::process::exit;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

type GridCell = i32;
type GridRow = Vec<GridCell>;
type Grid = Vec<GridRow>;

trait IterableGrid {
    fn grid_cons_iter(&self, len: usize) -> GridConsIter;
}

impl IterableGrid for Grid {
    fn grid_cons_iter(&self, len: usize) -> GridConsIter {
        GridConsIter {
            grid: self,
            len: len,
        }
    }
}

pub struct GridConsIter<'a> {
    grid: &'a Grid,
    len: usize,
}

impl<'a> Iterator for GridConsIter<'a> {
    type Item = &'a Vec<&'a i32>;

    fn next(&mut self) -> Option<&'a Vec<&'a i32>> {
        match self.grid.iter().next() {
            Some(row) => Some(row.iter().take(self.len).collect()),
            None => None,
        }
    }
}

fn main() {
    let grid_path = match args().nth(1) {
        Some(path) => path,
        None => {
            println!("Please provide an argument (path to a grid file)");
            exit(1);
        }
    };

    let grid = read_grid(grid_path);
    for i in grid.grid_cons_iter(4) {
        println!("{}", i);
    }
}

fn read_grid(file_path: String) -> Grid {
    let file = match File::open(&file_path) {
        Ok(f) => f,
        Err(e) => {
            println!("Can't read file at {}. Error was {}", file_path, e);
            exit(1);
        }
    };

    let reader = BufReader::new(file);
    let mut grid = Grid::new();
    for line in reader.lines() {
        match line {
            Ok(l) => grid.push(read_row(&l)),
            Err(e) => {
                println!("Error reading lines: {}", e);
                exit(1);
            }
        }
    }
    grid
}

fn read_row(string: &str) -> GridRow {
    string
        .split(' ')
        .map(|s|
            match i32::from_str_radix(s, 10) {
                Ok(n) => n,
                Err(e) => {
                    println!("Tried to parse `{}` as a number, but failed: {}", s, e);
                    exit(1);
                }
            }
        )
        .collect()
}

