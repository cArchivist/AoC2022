use std::fs;
use grid::Grid;
use std::cell::RefCell;

#[derive(Debug, Default)]
struct Tree {
    visible: bool,
    height: u32,
}

impl Tree {
    fn new(height: u32) -> Self {
        Self {
            visible: false,
            height: height,
        }
    }
}

fn visible_by_row(tree_grid: &mut Vec<Vec<Tree>>) {
    for row in 0..tree_grid.len() {
        let mut max_idx: usize = 0;
        let mut max_val: Option<u32> = None;
        for col in 0..tree_grid[row].len() { 
            let tree: &mut Tree = tree_grid.get_mut(row).unwrap().get_mut(col).unwrap();
            if max_val.is_none() || Some(tree.height) > max_val {
                tree.visible = true;
                max_idx = col;
                max_val = Some(tree.height);
            }
        }
        max_val = None;
        for col in (max_idx..tree_grid[row].len()).rev() { // reverse iterator stops at previously found max to limit double-work
            let tree: &mut Tree = tree_grid.get_mut(row).unwrap().get_mut(col).unwrap();
            if max_val.is_none() || Some(tree.height) > max_val {
                tree.visible = true;
                max_val = Some(tree.height);
            }
        }
    }
}

fn visible_by_column(tree_grid: &mut Vec<Vec<Tree>>) {
    for col in 0..tree_grid[0].len() {
        let mut max_idx: usize = 0;
        let mut max_val: Option<u32> = None;
        for row in 0..tree_grid.len() {
            let tree: &mut Tree = tree_grid.get_mut(row).unwrap().get_mut(col).unwrap();
            if max_val.is_none() || Some(tree.height) > max_val {
                tree.visible = true;
                max_idx = row;
                max_val = Some(tree.height);
            }
        }
        max_val = None;
        for row in (max_idx..tree_grid.len()).rev() {
            let tree: &mut Tree = tree_grid.get_mut(row).unwrap().get_mut(col).unwrap();
            if max_val.is_none() || Some(tree.height) > max_val {
                tree.visible = true;
                max_val = Some(tree.height);
            }
        }
    }
}

pub fn solve(filename: &str) -> usize {
    let mut tree_grid: Vec<Vec<Tree>> = vec![];
    fs::read_to_string(&filename).unwrap_or_else(|error| {
        panic!("error reading {}: {:?}", &filename, &error)
    })
    .lines()
    .for_each( |line| {
        let row: Vec<Tree> = line.chars().map(|c| Tree::new(c.to_digit(10).unwrap())).collect();
        tree_grid.push(row);
    });
    visible_by_column(&mut tree_grid);
    visible_by_row(&mut tree_grid);
    // tree_grid.iter().for_each(|row| println!("{:?}", row));
    tree_grid.iter().map(|row| row.iter().filter(|&t| t.visible).count()).sum()
}


struct Map2D {
    grid: Vec<Vec<usize>>,
}

impl Map2D {
    fn from(input: Vec<Vec<usize>>) -> Self {
        Self {
            grid: input
        }
    }

    fn scenic_score(&self, x: usize, y: usize) -> usize {
        if x == 0 || y == 0 || x == self.grid.len() - 1 || y == self.grid[x].len() - 1 {
            return 0;
        }

        let len = self.grid[x].len();
        let l:usize = 1 + self.grid[x][1..y].iter().rev().map_while(|&v| if v < self.grid[x][y] {Some(v) } else {None }).count() as usize;
        let r:usize = 1 + self.grid[x][y+1..len-1].iter().map_while(|&v| if v < self.grid[x][y] {Some(v) } else {None }).count() as usize;
        let u:usize = 1 + self.grid[1..x].iter().rev().map(|v| v[y]).map_while(|v| if v < self.grid[x][y] {Some(v) } else {None }).count() as usize;
        let d:usize = 1 + self.grid[x+1..len-1].iter().map(|v| v[y]).map_while(|v| if v < self.grid[x][y] {Some(v) } else {None }).count() as usize;
        l*r*u*d
    }

    fn max_scenic_score(&self) -> usize {
        self.grid.iter()
            .enumerate()
            .map(|(x, row)| row.iter().enumerate()
            .map(|(y, _)| self.scenic_score(x, y))
            .max()
            .unwrap())
            .max()
            .unwrap()
    }
}

pub fn solve2(filename: &str) -> usize {
    let mut tree_grid: Vec<Vec<usize>> = vec![];
    fs::read_to_string(&filename).unwrap_or_else(|error| {
        panic!("error reading {}: {:?}", &filename, &error)
    })
    .lines()
    .for_each( |line| {
        let row: Vec<usize> = line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
        tree_grid.push(row);
    });
    let grid = Map2D::from(tree_grid);
    grid.max_scenic_score()
}