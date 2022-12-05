use util::read_lines;

struct Elf {
    start: usize,
    end: usize
}

impl Elf {
    pub fn new(schedule: &str) -> Self {
        let parts: Vec<&str> = schedule.split("-").collect();
        Elf {
            start: parts[0].parse::<usize>().unwrap(),
            end: parts[1].parse::<usize>().unwrap()
        }
    }

    pub fn contains(&self, other: &Elf) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Elf) -> bool {
        !(self.start > other.end || self.end < other.start)
    }
}

pub fn solve(filename: &str) -> usize {
    let mut count: usize = 0;
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(content) = line {
                let v: Vec<&str> = content.split(",").collect();
                let e1 = Elf::new(v[0]);
                let e2 = Elf::new(v[1]);
                if e1.contains(&e2) || e2.contains(&e1) {
                    count += 1;
                }
            }
        }
    }

    count
}

pub fn solve2(filename: &str) -> usize {
    let mut count: usize = 0;
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(content) = line {
                let v: Vec<&str> = content.split(",").collect();
                let e1 = Elf::new(v[0]);
                let e2 = Elf::new(v[1]);
                if e1.overlaps(&e2) {
                    count += 1;
                }
            }
        }
    }

    count
}
