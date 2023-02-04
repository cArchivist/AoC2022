use std::fs;
use pathfinding::directed::bfs::bfs;

#[derive(Clone, Copy, Debug)]
struct Point {
    elevation: usize,
    dist: Option<usize>,
}

impl Point {
    fn from(mut elev_char: char) -> Self {
        if elev_char == 'S' {
            elev_char = 'a';
        } else if elev_char == 'E' {
            elev_char = 'z';
        }
        Point {
            elevation: elev_char as usize % 32,
            dist: None,
        }
    }

    fn valid_successor(self, np: Point) -> bool {
        if self.elevation >= np.elevation - 1 {
            return true
        }
        false
    }
}

struct Map2D {
    start: (usize, usize),
    end: (usize, usize),
    grid: Vec<Vec<Point>>,
}

impl Map2D {
    fn from(start: (usize, usize), end: (usize, usize), grid: Vec<Vec<Point>>) -> Self {
        Map2D { start: start, end: end, grid: grid }
    }

    fn get_successors(&self, start: (usize, usize)) -> Vec<(usize, usize)> {
        let mut successors = Vec::new();
        let diffs = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        let p = self.grid[start.0][start.1];
        diffs.into_iter().for_each(|(dx, dy)| {
            let x = (start.0 as isize + dx) as usize;
            let y = (start.1 as isize + dy) as usize;
            if self.in_grid((x, y)) {
                let new_p = self.grid[x][y];
                if p.valid_successor(new_p) {
                    successors.push((x, y));
                }
            }
        });
        successors
    }

    fn in_grid(&self, pos: (usize, usize)) -> bool {
        if pos.0 >= self.grid.len() {
            return false
        }
        if pos.1 >= self.grid[0].len() {
            return false
        }
        true
    }

    fn shortest_path(self) -> usize {
        match bfs(
            &self.start,
            |p| self.get_successors(*p),
            |p| *p==self.end
        ) {
            Some(v) => {
                // print!("{:?}", v);
                v.len() - 1 // don't include first node
            } ,
            None => 0
        }
    }

    // does what get_successors does, but in reversed direction (successor is valid if start would be a valid successor using existing fn)
    // has additional logic to skip if already scored to prevent needless iteration
    fn inverted_successors(&mut self, start: (usize, usize)) -> Vec<(usize, usize)> {
        let mut successors = Vec::new();
        let diffs = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        let p = self.grid[start.0][start.1];
        diffs.into_iter().for_each(|(dx, dy)| {
            let x = (start.0 as isize + dx) as usize;
            let y = (start.1 as isize + dy) as usize;
            if self.in_grid((x, y)) {
                let mut new_p = self.grid[x][y];
                if new_p.valid_successor(p) { // only if score is none or greater
                    if new_p.dist.is_none() || p.dist < Some(new_p.dist.unwrap_or(1000) - 1) {  // don't infinite loop if only 1 less (no change to total)
                        new_p.dist = Some(p.dist.unwrap_or(0) + 1);
                        self.grid[x][y] = new_p;
                        println!("New valid successor: {} {} {:?}", x, y, new_p);
                        successors.push((x, y));
                    }
                }
            }
        });
        successors
    }

    fn find_min(self) -> usize {
        self.grid.iter()
        .flatten()
        .filter(|p| p.elevation == 1 && p.dist.is_some())
        .map(|p| p.dist.unwrap())
        .min()
        .unwrap()
    }

    fn shortest_scenic(mut self) -> usize {
        let mut successors = vec![self.end];
        while let Some(top) = successors.pop() {
            successors.append(&mut self.inverted_successors((top.0, top.1)));
        }
        self.find_min()
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::{Point, Map2D};
    #[test]
    fn t_inv_succ() {
        let mut start: (usize, usize) = (0, 0);
        let mut end: (usize, usize) = (0, 0);
        let mut grid: Vec<Vec<Point>> = vec![];
        // data parsing
        fs::read_to_string("./src/test_grid.txt").unwrap_or_else(|error| {
            panic!("error reading {}: {:?}", "./src/test_grid.txt", &error)
        })
        .lines()
        .enumerate()
        .for_each(|(x, line)| {
            let row: Vec<Point> = line.chars().enumerate().map(|(y, c)| {
                if c == 'S' {
                    start = (x, y);
                } else if c == 'E' {
                    end = (x, y);
                }
                Point::from(c)
            }).collect();
            grid.push(row);
        });
        let mut m: Map2D = Map2D::from(start, end, grid);
        let mut succ = m.inverted_successors((1, 7));
        println!("{:?}", succ);
        assert!(succ.len() == 4);
        m.grid[1][6].dist = Some(3);
        succ = m.inverted_successors((1, 7));
        println!("{:?}", succ);
        assert!(succ.len() == 3);
        succ = m.inverted_successors((1, 3));
        println!("{:?}", succ);
        assert!(succ.len() == 2);
    }
}


/*
    For solve, find the shortest path between start and end, never increasing elevation by more
    than one unit at a time.
*/
pub fn solve(f: &str) -> usize {
    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);
    let mut grid: Vec<Vec<Point>> = vec![];
    // data parsing
    fs::read_to_string(&f).unwrap_or_else(|error| {
        panic!("error reading {}: {:?}", &f, &error)
    })
    .lines()
    .enumerate()
    .for_each(|(x, line)| {
        let row: Vec<Point> = line.chars().enumerate().map(|(y, c)| {
            if c == 'S' {
                start = (x, y);
            } else if c == 'E' {
                end = (x, y);
            }
            Point::from(c)
        }).collect();
        grid.push(row);
    });
    let m: Map2D = Map2D::from(start, end, grid);
    // solution
    m.shortest_path()

}


/*
    For solve2, find the shortest path from end to *ANY* minimum-elevation node, following traversal rules
    from previous (inverted).
*/
pub fn solve2(f: &str) -> usize {
    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);
    let mut grid: Vec<Vec<Point>> = vec![];
    // data parsing
    fs::read_to_string(&f).unwrap_or_else(|error| {
        panic!("error reading {}: {:?}", &f, &error)
    })
    .lines()
    .enumerate()
    .for_each(|(x, line)| {
        let row: Vec<Point> = line.chars().enumerate().map(|(y, c)| {
            if c == 'E' {
                end = (x, y);
            }
            Point::from(c)
        }).collect();
        grid.push(row);
    });
    let m: Map2D = Map2D::from(start, end, grid);
    m.shortest_scenic()
}