use util::read_lines;
use std::collections::HashMap;


pub struct Stacks {
    stacks: HashMap<usize, Vec<char>>
}

impl Stacks {
    fn new(init: Vec<&str>) -> Self {
        let stacks = Stacks::init_stacks(init);
        Stacks {
            stacks: stacks
        }
    }

    fn init_stacks(init: Vec<&str>) -> HashMap<usize, Vec<char>> {
        let mut h: HashMap<usize, Vec<char>> = HashMap::new();
        for i in 0..init.len() {
            h.insert(i+1, init[i].chars().collect());
        }
        h
    }

    fn move_crates(&mut self, count: usize, start: usize, end: usize) {
        for _ in 0..count {
            let to_move: char = self.stacks.get_mut(&start).unwrap().pop().unwrap();
            self.stacks.get_mut(&end).unwrap().push(to_move);
        }
    }

    fn move_clumps(&mut self, count: usize, start: usize, end: usize) {
        let mut staging: Vec<char> = Vec::new();
        for _ in 0..count {
            staging.push(self.stacks.get_mut(&start).unwrap().pop().unwrap());
        }
        for _ in 0..count {
            self.stacks.get_mut(&end).unwrap().push(staging.pop().unwrap());
        }
    }

    fn collect_output(&mut self) -> String {
        let mut v: Vec<char> = Vec::new();
        for i in 1..10 {
            v.push(self.stacks.get_mut(&i).unwrap().pop().unwrap());
        }
        v.iter().collect()
    }
}


pub fn solve(filename: &str) -> String {
    let init: Vec<&str> = vec!["JHGMZNTF", "VWJ", "GVLJBTH", "BPJNCDVL", "FWSMPRG", "GHCFBNVM", "DHGMR", "HNMVZD", "GNFH"];  // initial arrangement of crates
    let mut stacks = Stacks::new(init);
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(command) = line {
                let c_vec: Vec<&str> = command.split(" ").collect();
                let count = c_vec[1].parse::<usize>().unwrap();
                let start = c_vec[3].parse::<usize>().unwrap();
                let end = c_vec[5].parse::<usize>().unwrap();
                stacks.move_clumps(count, start, end)
            }
        }
    }
    stacks.collect_output()
}
