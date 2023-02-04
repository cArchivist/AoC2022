use std::fs;

struct Command {
    com: String,
    amount: isize,
}

impl Command {
    fn from(com: &str, amount: isize) -> Self {
        Self {
            com: com.to_string(),
            amount: amount,
        }
    }

    fn noop() -> Self {
        Self {
            com: "noop".to_string(),
            amount: 0,
        }
    }
}

pub fn solve(filename: &str) -> isize {
    let key_cycles: Vec<usize> = vec![20, 60, 100, 140, 180, 220];
    let mut sum: isize = 0;
    let mut commands: Vec<Command> = Vec::new();
    fs::read_to_string(filename).unwrap_or_else(|error| {
        panic!("error reading {}: {:?}", &filename, &error)
    })
    .lines()
    .for_each(|line| {
        let v: Vec<&str> = line.split_whitespace().collect();
        match v[0] {
            "noop" => {
                commands.push(Command::noop());
            },
            "addx" => {
                commands.push(Command::noop()); // fake noop spacer to account for cycle delay
                commands.push(Command::from(v[0], v[1].parse::<isize>().unwrap()));
            },
            _ => {}
        }
    });
    let mut i: usize = 1;
    let mut reg: isize = 1;
    while i < 221 {
        let com = &commands[i-1];
        match com.com.as_str() {
            "noop" => {},
            "addx" => {
                reg = reg + com.amount;
            },
            _ => {}
        }
        if key_cycles.contains(&i) {
            println!("Current values: i: {}, reg: {}", i, reg);
            sum = sum + (i as isize * reg);
        }
        i = i + 1;
    }
    sum
}

pub fn coached_solve(filename: &str) -> usize {
    fs::read_to_string(filename).unwrap().lines()
    .fold(vec![1], |mut acc, curr| {
        let last = *acc.last().unwrap();
        acc.push(last);
        if curr[..4] != *"noop" {
            let val = curr.split(" ").last().unwrap().parse::<isize>().ok().unwrap();
            acc.push(last + val);
        };
        acc
    })
    .iter()
    .enumerate()
    .filter(|i| (i.0+1) % 40 == 20)
    .map(|a| ((a.0 as isize) + 1) * a.1)
    .sum::<isize>() as usize
}

pub fn solve2(filename: &str) -> usize {
    let v: Vec<&str> = fs::read_to_string(filename).unwrap().lines()
    .fold(vec![1], |mut acc, curr| {
        let last = *acc.last().unwrap();
        acc.push(last);
        if curr[..4] != *"noop" {
            let val = curr.split(" ").last().unwrap().parse::<isize>().ok().unwrap();
            acc.push(last + val);
        };
        acc
    })
    .iter()
    .enumerate()
    .map(|a| ((a.0 % 40).abs_diff(*a.1 as usize) <= 1))
    .map(|b| {
        match b {
            true => "#",
            false => ".",
        }
    })
    .collect::<Vec<&str>>();
    for i in 0..6 {
        println!("{:?}", v[i*40..i*40+39].into_iter().map(|s| s.to_string()).collect::<String>());
    }

    0
}