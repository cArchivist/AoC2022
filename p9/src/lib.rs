use util::read_lines;
use std::fs;
use std::collections::HashSet;
use std::ops::Add;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn origin() -> Self {
        Self {
            x: 0,
            y: 0,
        }
    }
    fn from(t: (isize, isize)) -> Self {
        Self {
            x: t.0,
            y: t.1,
        }
    }
    fn parse(s: &str) -> Self {
        match s {
            "U" => Point{x: 0, y: 1},
            "R" => Point{x: 1, y: 0},
            "L" => Point{x: -1, y: 0},
            "D" => Point{x: 0, y: -1},
            _ => Point{x: 0, y: 0},
        }
    }

    fn adjacent(&self, other: &Point) -> bool {
        self.x.abs_diff(other.x) < 2 && self.y.abs_diff(other.y) < 2
    }

    fn correction(&self, other: &Point) -> Point {
        let x_c: isize = match Some(other.x - self.x) {
            Some(x) if x < 0 => -1,
            Some(x) if x > 0 => 1,
            _ => 0,
        };
        let y_c: isize = match Some(other.y - self.y) {
            Some(x) if x < 0 => -1,
            Some(x) if x > 0 => 1,
            _ => 0,
        };
        Point {
            x: x_c,
            y: y_c,
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Point{x: self.x + other.x, y: self.y + other.y}
    }
}

struct Move {
    dir: Point,
    dist: isize
}

impl Move {
    fn from(m: Point, d: isize) -> Self {
        Self {
            dir: m,
            dist: d
        }
    }
}

struct Rope {
    knot_pos: Vec<Point>,
    t_visited: HashSet<Point>
}

impl Rope {
    fn new(k: usize) -> Self {
        let mut h: HashSet<Point> = HashSet::new();
        h.insert(Point::origin());
        Self {
            knot_pos: vec![Point::origin(); k],
            t_visited: h
        }
    }
    fn make_move(&mut self, m: &Move) {
        for _ in 0..m.dist {
            self.knot_pos[0] = self.knot_pos[0] + m.dir;
            for j in 1..self.knot_pos.len() {
                self.update_pos(j);
            }
            println!("Knots: {:?}", self.knot_pos);
        }
    }

    fn update_pos(&mut self, j: usize) {
        if !self.knot_pos[j-1].adjacent(&self.knot_pos[j]) {
            self.correct(j);
            if j == self.knot_pos.len()-1 {
                self.t_visited.insert(self.knot_pos[self.knot_pos.len()-1]);
            }
        }
    }

    fn correct(&mut self, j: usize) {
        self.knot_pos[j] = self.knot_pos[j] + self.knot_pos[j].correction(&self.knot_pos[j-1]);
    }

    fn visited(&self) -> usize {
        self.t_visited.len()
    }
}

pub fn solve(filename: &str) -> usize {
    let mut rope: Rope = Rope::new(10);
    let mut movements: Vec<Move> = Vec::new();
    fs::read_to_string(filename).unwrap_or_else(|error| {
        panic!("error reading {}: {:?}", &filename, &error)
    })
    .lines()
    .for_each( |line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let m = Point::parse(parts[0]);
        let d: isize = parts[1].parse::<isize>().unwrap();
        movements.push(Move::from(m, d))
    });
    movements.iter().for_each(|m| rope.make_move(m));
    // print!("Visited locations: {:?}", rope.t_visited);
    rope.visited()
}