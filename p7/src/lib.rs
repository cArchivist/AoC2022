use std::fs;
use std::collections::HashMap;
use regex::Regex;

const MAX_SIZE: u32 = 100000;
const MIN_FREE_SIZE: u32 = 30000000;
const TOTAL_SIZE: u32 = 70000000;

pub fn solve(filename: &str) -> u32 {
    let mut curr_path: Vec<String> = vec![];  // tracks current dir hierarchy (join for full)
    let mut dirsizes: HashMap<String, u32> = HashMap::<String, u32>::new(); // hashmap of directories
    let cd_regex = Regex::new(r"^\$ cd (\S+)").unwrap();    // regex parsing cd commands only
    let file_regex = Regex::new(r"^(\d+) (\S+)").unwrap();  // regex parsing file sizes only
    fs::read_to_string(&filename).unwrap_or_else( |error| {
            panic!("error reading {}: {:?}", &filename, &error) // err handling on file read
        })
        .lines()
        .for_each( |line| {
            if cd_regex.is_match(line) { // if cd command
                let caps = cd_regex.captures(line).unwrap(); // get capture groups
                let mut dirname = caps.get(1).unwrap().as_str().to_string(); // first capture group
                if dirname == ".." {
                    curr_path.pop(); // remove current path from hierarchy
                } else {
                    if dirname == "/" {
                        dirname = "<root>".to_string();
                    }
                    curr_path.push(dirname);
                    let path_str = curr_path.join("/"); // condense current path into single string for hashmap indexing
                    if ! dirsizes.contains_key(&path_str) {
                        dirsizes.insert(path_str, 0);
                    }
                }
            } else if file_regex.is_match(line) { // if file line
                let caps = file_regex.captures(line).unwrap();  // get capture groups
                let filesize: u32 = caps.get(1).unwrap().as_str().parse().unwrap();  // first capture group
                let mut update_list: Vec<String> = vec![]; // make temporary vec for path algebra
                for dir in &curr_path { // for every directory in the path
                    update_list.push(dir.to_string()); 
                    let update_path = update_list.join("/"); // create key
                    dirsizes.entry(update_path).and_modify(|size| *size += filesize); // update size
                }
            }
        });
    let curr_free: u32 = TOTAL_SIZE - dirsizes["<root>"];
    let need_to_delete: u32 = MIN_FREE_SIZE - curr_free;
    let total: u32 = *dirsizes.values().filter(|&s| *s > need_to_delete).min().unwrap();
    total
}
