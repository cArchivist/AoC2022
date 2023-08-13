#[derive(Debug,Eq,Ord)]
struct List {
    items: Vec<Signal>,
}

#[derive(Debug,Eq,Ord)]
enum Signal {
    Integer(i64),
    List(Box<List>),
}

impl PartialEq for Signal {
    fn eq(&self, other: &Signal) -> bool {
        match(self, other) {
            (&Signal::Integer(ref a), &Signal::Integer(ref b)) => a == b,
            (&Signal::List(ref a), &Signal::List(ref b)) => a == b,
            _ => false,
        }
    }
}

impl PartialEq for List {
    fn eq(&self, other: &List) -> bool {
        self.items == other.items
    }
}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.items.partial_cmp(&other.items)
    }
}

impl PartialOrd for Signal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (&Signal::Integer(ref a), &Signal::Integer(ref b)) => a.partial_cmp(b),
            (&Signal::List(ref a), &Signal::List(ref b)) => a.partial_cmp(b),
            (&Signal::Integer(ref a), &Signal::List(ref b)) => Box::new(List { items: vec![Signal::Integer(*a)] }).partial_cmp(b),
            (&Signal::List(ref a), &Signal::Integer(ref b)) => a.partial_cmp(&Box::new(List { items: vec![Signal::Integer(*b)] })),
        }
    }
}

fn parse_list(input: &str) -> Result<(List, &str), &str> {
    let mut items = vec![];
    let mut input = input.trim_start();

    if !input.starts_with("[") {
        return Err("Expected '[' at start of list");
    }
    input = &input[1..];

    while !input.starts_with("]") {
        let (item, rest) = parse_signal(input)?;
        items.push(item);
        input = rest.trim_start();

        if input.starts_with(",") {
            input = &input[1..];
        }
    }
    input = &input[1..];

    Ok((List { items }, input))
}

fn parse_signal(input: &str) -> Result<(Signal, &str), &str> {
    if input.starts_with("[") {
        parse_list(input).map(|(list, rest)| (Signal::List(Box::new(list)), rest))
    } else {
        parse_integer(input)
    }
}

fn parse_integer(input: &str) -> Result<(Signal, &str), &str> {
    let mut end = 0;
    while end < input.len() && input.as_bytes()[end] >= b'0' && input.as_bytes()[end] <= b'9' {
        end += 1;
    }
    let (integer_str, rest) = input.split_at(end);
    let integer = integer_str.parse::<i64>().map_err(|_| "Expected integer")?;
    Ok((Signal::Integer(integer), rest))
}

fn compare(left: &str, right: &str) -> bool {
    let left = parse_list(left);
    let right = parse_list(right);

    match(left, right) {
        (Ok((left, _)), Ok((right, _))) => left < right,
        _ => false,
    }
}

/*
    For solve, compare each given pair of lines, then count the number which are correctly ordered.
 */
pub fn solve() -> usize {
    let input = include_str!("p13.txt").split("\r\n\r\n").enumerate(); // lol carriage return
    
    let k:usize = input.map(|(i, group)| {
        let group_number = i+1;
        let left = group.split("\n").next().unwrap();
        let right = group.split("\n").last().unwrap();
        // println!("left: {:?}, right: {:?}", left, right);
        (group_number, compare(left, right))
    }).filter(|(_, x)| *x)
        .map(|(i, _)| i)
        .sum();
    k
}

pub fn solve2() -> usize {
    let mut x = include_str!("p13.txt").lines().filter(|x| !x.eq(&"")).map(str::trim).collect::<Vec<&str>>();
    x.append(&mut vec!["[[2]]"]);
    x.append(&mut vec!["[[6]]"]);

    let mut x = x.iter()
        .map(|x| parse_list(x))
        .map(|x| x.ok().unwrap().0)
        .collect::<Vec<List>>();
    x.sort();

    let a = parse_list("[[2]]").ok().unwrap().0;
    let b = parse_list("[[6]]").ok().unwrap().0;

    let a = x.iter().position(|x| x.eq(&a)).unwrap();
    let b = x.iter().position(|x| x.eq(&b)).unwrap();

    (a+1) * (b+1)
}