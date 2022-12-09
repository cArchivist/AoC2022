use util::RingBuffer;
use std::fs;
use std::collections::HashSet;

pub fn solve(filename: &str, size: usize) -> usize {
    let data = fs::read_to_string(filename).expect("Couldn't read file");
    let char_vec: Vec<char> = data.chars().collect();
    let mut buffer: RingBuffer<char> = RingBuffer::with_capacity(size);
    let mut idx: usize = 0;
    loop {
        buffer.push(char_vec[idx]);
        idx += 1; // this way it will be in sync with character count
        println!("Current index: {}", idx);
        println!("Current buffer contents: {:?}", buffer.vec);
        if buffer.is_full() {
            if buffer.all_unique() {
                return idx
            }
        }
    }
}