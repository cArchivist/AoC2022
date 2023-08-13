use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};


#[derive(Clone, Copy, Debug)]
struct Spot {
    occupied: bool,
    fill_type: u8, // 0: rock, 1: sand
}

const ROCK: u8 = 0;
const SAND: u8 = 1;

const FALL_DOWN: (isize, isize) = (0, 1);
const FALL_LEFT: (isize, isize) = (-1, 1);
const FALL_RIGHT: (isize, isize) = (1, 1);
const BLOCKED: (isize, isize) = (0, 0);

const FALL_PROGRESSION: [(isize, isize); 4] = [FALL_DOWN, FALL_LEFT, FALL_RIGHT, BLOCKED];

trait KeyPair<A, B> {
    fn a(&self) -> &A;
    fn b(&self) -> &B;
}

impl <'a, A, B> Borrow<dyn KeyPair<A, B> + 'a> for (A, B)
where
    A: Eq + Hash + 'a,
    B: Eq + Hash + 'a,
{
    fn borrow(&self) -> &(dyn KeyPair<A, B> + 'a) {
        self
    }
}

impl<A: Hash, B: Hash> Hash for dyn KeyPair<A, B> + '_ {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.a().hash(state);
        self.b().hash(state);
    }
}

impl<A: Eq, B: Eq> PartialEq for dyn KeyPair<A, B> + '_ {
    fn eq(&self, other: &Self) -> bool {
        self.a() == other.a() && self.b() == other.b()
    }
}

impl<A: Eq, B: Eq> Eq for dyn KeyPair<A, B> + '_ {}

#[derive(Debug)]
struct CaveGrid {
    map: HashMap<(isize, isize), Spot>, // x, y but not 0-indexed
    lowest: isize, // this is the max Y position represented by a rock, i.e., if sand passes this point, it must be falling infinitely
    sand_origin: (isize, isize) // where the sand falls from
}

impl CaveGrid {
    fn new() -> Self {
        CaveGrid {
            map: HashMap::new(),
            lowest: 0,
            sand_origin: (500, 0),
        }
    }

    fn get(&self, a: &isize, b: &isize) -> Spot {
        *self.map.get(&(a, b) as &dyn KeyPair<isize, isize>).unwrap()
    }

    fn set(&mut self, a: isize, b: isize, v: Spot) {
        self.map.insert((a, b), v);
    }

    fn fill_between(&mut self, start: &(isize, isize), end: &(isize, isize), fill_type: u8) {
        println!("Filling between {:?} and {:?}", start, end);
        if start.0 != end.0 {
            for i in start.0..=end.0 { // need to include end space
                self.map.insert((i, start.1), Spot{ occupied: true, fill_type} );
            }
            if start.1 > self.lowest {
                self.lowest = start.1
            }
        } else if start.1 != end.1 {
            for i in start.1..=end.1 { // TODO: fix this range to work in two directions
                self.map.insert((start.0, i), Spot{ occupied: true, fill_type} );
            }
            let lower = start.1 < end.1;
            let take = match lower {
                true => end.1,
                false => start.1,
            };
            if take > self.lowest { 
                self.lowest = take;
            }
        }
    }

    fn fill_rocks(&mut self, input: &str) {
        let v: Vec<(isize, isize)> = input.split(" -> ").map(|pair| {
            to_uint_tuple(pair)
        }).collect();
        for i in 0..v.len() - 1 {
            let start = v[i];
            let end = v[i+1];
            self.fill_between(&start, &end, ROCK);
        }
    }

    /* fill_sand needs to cover the following cases:
        1) in free fall (check down, keep going)
        2) stop and accumulate (occupied below, below-left, below-right)
        3) shift (occupied below | below-left but not everywhere)
        4) abyss free fall (no more catches below)
     */ 
    fn fill_sand(&mut self) {
        let mut sand_pos = self.sand_origin;
        let mut mound_tip = sand_pos; // represents the first spot the sand diverged from free fall
        let mut fall_idx = 0;
        while sand_pos.1 < self.lowest {
            let next = (sand_pos.0 + FALL_PROGRESSION[fall_idx].0, sand_pos.1 + FALL_PROGRESSION[fall_idx].1);
            match self.map.get(&next) {
                Some(_) => {
                    fall_idx = fall_idx + 1;
                    if fall_idx == 1 { // newly stopped, i.e. new mound
                        mound_tip = sand_pos;
                        println!("New mound at {:?}", mound_tip);
                    } else if fall_idx == FALL_PROGRESSION.len() { // fully blocked, i.e. deposit sand at current position
                        self.map.insert((sand_pos.0, sand_pos.1), Spot{ occupied: true, fill_type: SAND });
                        sand_pos = mound_tip; // reset to the mound tip
                        fall_idx = 1; // at the tip of the mound, start by falling left (because a mound means you can't fall down)
                        continue
                    } 
                },
                None => { // back to free fall from new point
                    sand_pos = next;
                    fall_idx = 0;
                },
            };
            println!("Next spot: {:?}", sand_pos);
        }
    }

}

impl<A, B> KeyPair<A, B> for (A, B) {
    fn a(&self) -> &A {
        &self.0
    }
    fn b(&self) -> &B {
        &self.1
    }
}

impl<A, B> KeyPair<A, B> for (&A, &B) {
    fn a(&self) -> &A {
        self.0
    }
    fn b(&self) -> &B {
        self.1
    }
}

fn to_uint_tuple(s: &str) -> (isize, isize) {
    let v: Vec<&str> = s.split(",").collect();
    let a: isize = v[0].parse::<isize>().unwrap();
    let b: isize = v[1].parse::<isize>().unwrap();
    (a, b)
}

pub fn solve() -> usize {
    let mut grid = CaveGrid::new();
    include_str!("test.txt").lines().enumerate().for_each(|(_, line)| {
        grid.fill_rocks(line)
    });
    println!("Base grid state: {:?}", grid);
    grid.fill_sand();
    grid.map.into_iter().fold(0, |acc, x| if x.1.fill_type == SAND { acc + 1 } else { acc })
}