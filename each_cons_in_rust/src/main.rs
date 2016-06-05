use std::iter::Iterator;

struct EachConsIterator<'a> {
    vector: &'a Vec<i32>,
    len: usize,
    pos: usize,
}

impl<'a> Iterator for EachConsIterator<'a> {
    type Item = &'a Vec<i32>;

    fn next(&mut self) -> Option<&'a Vec<i32>> {

        None
    }
}

trait EachConsIterable<'a> {
    fn each_cons(&'a self, len: usize) -> EachConsIterator<'a>;
}

impl<'a> EachConsIterable<'a> for Vec<i32> {
    fn each_cons(&'a self, len: usize) -> EachConsIterator<'a> {
        EachConsIterator {
            vector: &self,
            len: len,
            pos: 0,
        }
    }
}

fn main() {
    let v = vec![1, 2, 3, 4, 5, 6];
    for w in v.each_cons(3) {
        println!("* Consecutives");
        for n in w {
            println!("{}", n)
        }
    }
    println!("Done!");
}
