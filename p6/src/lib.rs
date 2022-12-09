use ringbuffer::{AllocRingBuffer, RingBufferWrite, RingBufferRead, RingBuffer, RingBufferExt};
use std::fs;
use std::collections::HashSet;

fn all_unique(buffer: &impl RingBufferExt<char>, size: usize) -> bool {
    let mut h: HashSet<char> = HashSet::new();
    let idx_size: isize = size.try_into().unwrap();
    for i in 0..idx_size {
        h.insert(*buffer.get(i).unwrap());
    }
    println!("{:?}", h);
    if h.len() == size.try_into().unwrap() {
        return true
    }
    false
}

pub fn solve(filename: &str, size: usize) -> usize {
    let data = fs::read_to_string(filename).expect("Couldn't read file");
    let char_vec: Vec<char> = data.chars().collect();
    let mut buffer: AllocRingBuffer<char> = AllocRingBuffer::with_capacity(size);
    let mut idx: usize = 0;
    loop {
        buffer.push(char_vec[idx]);
        idx += 1; // this way it will be in sync with character count
        if buffer.is_full() {
            if all_unique(&buffer, size) {
                return idx
            }
        }
    }
}