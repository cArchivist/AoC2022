

pub fn solve(filename: &str) -> usize {
    let mut v: Vec<usize> = Vec::new();
    let mut temp: usize = 0;
    match std::fs::read_to_string(filename) {
        Ok(lines) => {
            for line in lines.as_str().lines() {
                if line.is_empty() {
                    v.push(temp);
                    temp = 0;
                } else {
                    let int_val = line.parse::<usize>().unwrap();
                    temp += int_val;
                }
            }
        }
        Err(_) => (),
    }
    *v.iter().max().unwrap()
}

pub fn solve2(filename: &str) -> usize {
    let mut v: Vec<usize> = Vec::new();
    let mut temp: usize = 0;
    match std::fs::read_to_string(filename) {
        Ok(lines) => {
            for line in lines.as_str().lines() {
                if line.is_empty() {
                    v.push(temp);
                    temp = 0;
                } else {
                    let int_val = line.parse::<usize>().unwrap();
                    temp += int_val;
                }
            }
        }
        Err(_) => (),
    }
    v.sort();
    let len = v.len();
    let top3 = &v[len-3..len];
    top3[0] + top3[1] + top3[2]
}