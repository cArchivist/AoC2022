use util::read_lines;
use std::collections::HashMap;

struct ScoreTable {
    pub base: usize,
    pub matchups: HashMap<String, usize>,
}

pub fn solve(filename: &str) -> usize {
    let mut score: usize = 0;
    let score_map = init_score_map();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(content) = line {
                let v: Vec<&str> = content.split(" ").collect();
                let opp = v[0];
                let player = v[1];
                let round_score = score_map[player].base + score_map[player].matchups[opp];
                score += round_score;
            }
        }
    }
    score
}

fn init_score_map() -> HashMap<String, ScoreTable> {
    let score_map: HashMap<String, ScoreTable> = HashMap::from([
        ("X".to_string(), ScoreTable { // you play rock
            base: 1,
            matchups: HashMap::from([
                (String::from("A"), 3), // rock
                (String::from("B"), 0), // paper
                (String::from("C"), 6), // scissors
            ]),
        }),
        ("Y".to_string(), ScoreTable { // you play paper
            base: 2,
            matchups: HashMap::from([
                (String::from("A"), 6),
                (String::from("B"), 3),
                (String::from("C"), 0),
            ]),
        }),
        ("Z".to_string(), ScoreTable { // you play scissors
            base: 3,
            matchups: HashMap::from([
                (String::from("A"), 0),
                (String::from("B"), 6),
                (String::from("C"), 3),
            ])
        }),
    ]);
    score_map
}

pub fn solve2(filename: &str) -> usize {
    let mut score: usize = 0;
    let score_map = init_score_map2();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(content) = line {
                let v: Vec<&str> = content.split(" ").collect();
                let opp = v[0];
                let player = v[1];
                let round_score = score_map[player].base + score_map[player].matchups[opp];
                score += round_score;
            }
        }
    }
    score
}

fn init_score_map2() -> HashMap<String, ScoreTable> {
    let score_map: HashMap<String, ScoreTable> = HashMap::from([
        ("X".to_string(), ScoreTable { // lose
            base: 0,
            matchups: HashMap::from([
                (String::from("A"), 3), // opp rock, play scissors (C)
                (String::from("B"), 1), // opp paper, play rock (A)
                (String::from("C"), 2), // opp scissors, play paper (B)
            ]),
        }),
        ("Y".to_string(), ScoreTable { // draw
            base: 3,
            matchups: HashMap::from([
                (String::from("A"), 1),
                (String::from("B"), 2),
                (String::from("C"), 3),
            ]),
        }),
        ("Z".to_string(), ScoreTable { // win
            base: 6,
            matchups: HashMap::from([
                (String::from("A"), 2),
                (String::from("B"), 3),
                (String::from("C"), 1),
            ])
        }),
    ]);
    score_map
}