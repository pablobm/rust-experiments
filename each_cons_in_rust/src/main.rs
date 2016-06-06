use std::iter::Iterator;

struct AllConsIter<'a> {
    vector: &'a Vec<i32>,
    len: usize,
    next_start: usize,
}

impl<'a> Iterator for AllConsIter<'a> {
    type Item = Consecutives<'a>;

    fn next(&'a mut self) -> Option<Self::Item> {
        if self.next_start < self.vector.len() - self.len {
            let start = self.next_start;
            self.next_start += 1;
            let cons = Consecutives {
                all: self,
                start: start,
            };
            Some(cons)
        }
        else {
            None
        }
    }
}

struct Consecutives<'a> {
    all: &'a AllConsIter<'a>,
    start: usize,
}

impl<'a> Consecutives<'a> {
    fn len(&self) -> usize {
        self.all.len
    }

    fn iter(&self) -> SingleConsIter {
        SingleConsIter {
            cons: self,
            pos: self.start,
        }
    }
}

struct SingleConsIter<'a> {
    cons: &'a Consecutives<'a>,
    pos: usize,
}

impl<'a> Iterator for SingleConsIter<'a> {
    type Item = &'a i32;

    fn next(&mut self) -> Option<&'a i32> {
        if self.pos < self.cons.len() {
            let yielding_pos = self.pos;
            self.pos += 1;
            Some(&self.cons.all.vector[yielding_pos])
        }
        else {
            None
        }
    }
}

trait EachConsIterable<'a> {
    fn each_cons(&'a self, len: usize) -> AllConsIter<'a>;
}

impl<'a> EachConsIterable<'a> for Vec<i32> {
    fn each_cons(&'a self, len: usize) -> AllConsIter<'a> {
        AllConsIter {
            vector: self,
            len: len,
        }
    }
}

fn main() {
    let v = vec![1, 2, 3, 4, 5, 6];
    for cons in v.each_cons(3) {
        println!("* Consecutives");
        for n in cons.iter() {
            println!("{}", n)
        }
    }
    println!("Done!");
}
