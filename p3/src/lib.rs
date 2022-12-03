use util::read_lines;
use std::collections::HashSet;

pub fn solve(filename: &str) -> usize {
    let alphabet: String = String::from_utf8((b'a'..=b'z').chain(b'A'..=b'Z').collect()).unwrap();
    let mut sum: usize = 0;
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ruck) = line {
                let (pouch1, pouch2) = ruck.split_at(ruck.len() / 2);
                let p1_set = hash_set_from_str(pouch1);
                println!("Unique contents from pouch 1: {:?}", &p1_set);
                let p2_set = hash_set_from_str(pouch2);
                println!("Unique contents from pouch 2: {:?}", &p2_set);
                let intersection: Vec<&char> = p1_set.intersection(&p2_set).collect();
                println!("Intersection of pouches: {:?}", &intersection);
                let p = priority(*intersection[0], &alphabet);
                println!("Computed priority: {}", &p);
                sum += p;
            }
        }
    }
    sum
}

fn hash_set_from_str(s: &str) -> HashSet<char> {
    let mut h: HashSet<char> = HashSet::new();
    s.chars().for_each(|c| {
        h.insert(c);
    });
    h
}

fn priority(c: char, alpha: &String) -> usize {
    alpha.find(c).unwrap() + 1
}

pub fn solve2(filename: &str) -> usize {
    let alphabet: String = String::from_utf8((b'a'..=b'z').chain(b'A'..=b'Z').collect()).unwrap();
    let mut sum: usize = 0;
    if let Ok(mut lines) = read_lines(filename) {
        loop {
            let c = lines.next();
            if c.is_none() {
                break;
            }
            let pouch1 = c.unwrap().unwrap();
            let p1_set = hash_set_from_str(&pouch1);
            let pouch2 = lines.next().unwrap().unwrap();
            let p2_set = hash_set_from_str(&pouch2);
            let pouch3 = lines.next().unwrap().unwrap();
            let p3_set = hash_set_from_str(&pouch3);
            let i_full = p1_set.intersection(&p2_set).find(|it| p3_set.contains(it)).unwrap(); // 3-way intersection
            sum += priority(*i_full, &alphabet);
        }
    }
    sum
}