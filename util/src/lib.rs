use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use std::hash::Hash;
use std::fmt::Debug;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub struct RingBuffer<T> where T: Eq + Hash + Copy + Debug {
    pub vec: Vec<T>,
    size: usize,
    idx: usize
}

impl <T>RingBuffer<T> where T: Eq + Hash + Copy + Debug {
    pub fn with_capacity(size: usize) -> Self {
        Self{
            idx: 0,
            size: size,
            vec: Vec::with_capacity(size)
        }
    }

    pub fn push(&mut self, element: T) {
        if self.vec.len() <= self.idx {
            self.vec.push(element);
        } else {
            self.vec[self.idx] = element;
        }
        self.idx += 1;
        if self.idx == self.size {
            self.idx = 0;
        }
    }

    pub fn all_unique(&self) -> bool {
        let mut h: HashSet<T> = HashSet::new();
        for c in self.vec.iter() {
            h.insert(*c);
        }
        println!("{:?}", h);
        if h.len() == self.size {
            return true
        }
        false
    }

    pub fn is_full(&self) -> bool {
        self.vec.len() == self.size
    }
}