use std::collections::VecDeque;

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    op: fn(u64) -> u64,
    test: (u64, u64, u64),
    handles: u64
}


impl Monkey {
    fn from(v: VecDeque<u64>, op: fn(u64) -> u64, test: u64, t_target: u64, f_target: u64) -> Self {
        Self {
            items: v,
            op: op,
            test: (test, t_target, f_target),
            handles: 0
        }
    }
}

fn init_monkeys() -> Vec<Monkey> {
    let mut v: Vec<Monkey> = Vec::new();
        v.push(Monkey::from(VecDeque::from(vec![84, 66, 62, 69, 88, 91, 91]), |a| a*11,2, 4, 7));
        v.push(Monkey::from(VecDeque::from(vec![98, 50, 76, 99]), |a| a*a, 7, 3, 6));
        v.push(Monkey::from(VecDeque::from(vec![72, 56, 94]), |a| a+1, 13, 4, 0));
        v.push(Monkey::from(VecDeque::from(vec![55, 88, 90, 77, 60, 67]), |a| a + 2, 3, 6, 5));
        v.push(Monkey::from(VecDeque::from(vec![69, 72, 63, 60, 72, 52, 63, 78]), |a| a*13, 19, 1, 7));
        v.push(Monkey::from(VecDeque::from(vec![89, 73]), |a| a + 5, 17, 2, 0));
        v.push(Monkey::from(VecDeque::from(vec![78, 68, 98, 88, 66]), |a| a + 6, 11, 2, 5));
        v.push(Monkey::from(VecDeque::from(vec![70]), |a| a + 7, 5, 1, 3));
    v
}


fn gcd(first: u64, second: u64) -> u64 {
    let (mut max, mut min) = (first, second);
    if min < max {
        (min, max) = (max, min);
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        (max, min) = (min, res);
    }
}

fn lcm(first: u64, second: u64) -> u64 {
    first * second / gcd(first, second)
}

fn run_round(monkeys: &mut Vec<Monkey>, lcm: Option<u64>) {
    for j in 0..monkeys.len() {
        loop {
            let mut item = match monkeys[j].items.pop_front() {
                None => break,
                Some(item) => item,
            };

            monkeys[j].handles += 1;
            item = (monkeys[j].op)(item);
            if let Some(lcm) = lcm {
                item = item % lcm;
            } else {
                item = item / 3;
            }

            let t = item % monkeys[j].test.0 == 0;
            let m_new = if t {
                monkeys[j].test.1
            } else {
                monkeys[j].test.2
            };
            monkeys[m_new as usize].items.push_back(item);
        }
    }
}

pub fn solve() -> u64 {
    let mut monkeys: Vec<Monkey> = init_monkeys();
    let mut curr_lcm = 1;
    let mods = vec![2, 3, 5, 7, 11, 13, 17, 19];  // set up Chinese Remainder Theorem
    for i in 0..mods.len() {
        curr_lcm = lcm(curr_lcm, mods[i]);
    }
    for _ in 0..10000 {
        run_round(&mut monkeys, Some(curr_lcm));
    }
    monkeys.sort_by(|a, b| b.handles.partial_cmp(&a.handles).unwrap());
    println!("{:?}", monkeys);
    monkeys[0].handles * monkeys[1].handles
}