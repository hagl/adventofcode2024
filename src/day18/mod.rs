use std::{collections::HashSet, fs};

use crate::grid::{Direction, Grid, Point};

fn task1(positions: &Vec<(usize, usize)>, w: usize, h: usize, t: usize) -> usize {
    run(positions, w, h, t).unwrap()
}

// f(lower) = false, f(upper) = true;
fn binary_search<P>(lower: usize, upper: usize, f: P) -> usize
where
    P: Fn(usize) -> bool,
{
    let d = upper - lower;
    if d == 1 {
        upper
    } else {
        let mid = lower + (d / 2);
        if f(mid) {
            binary_search(lower, mid, f)
        } else {
            binary_search(mid, upper, f)
        }
    }
}

fn task2(positions: &Vec<(usize, usize)>, w: usize, h: usize, t: usize) -> String {
    let ix = binary_search(t, positions.len() - 1, |ix| {
        run(positions, w, h, ix).is_none()
    });
    let brick = positions[ix - 1];
    format!("{},{}", brick.0, brick.1)
    /*
        if let Some(ix) = (t..positions.len()).find(|ix| run(positions, w, h, *ix).is_none()) {
            let brick = positions[ix - 1];
            format!("{},{}", brick.0, brick.1)
        } else {
            "nof found".to_string()
        }
    */
}

fn run(positions: &Vec<(usize, usize)>, w: usize, h: usize, t: usize) -> Option<usize> {
    let mut grid: Grid = Grid::from_array(&vec![vec!['.'; w]; h]);
    for &(x, y) in positions.iter().take(t) {
        grid.set_point(Point { x, y }, '#');
    }

    let start = Point { x: 0, y: 0 };
    let end = Point { x: w - 1, y: h - 1 };

    // let mut minimums: HashMap<Point, usize> = HashMap::new();
    // minimums.insert(start, 0);
    let mut visited: HashSet<Point> = HashSet::new();
    let mut todo: HashSet<Point> = HashSet::new();

    visited.insert(start);
    todo.insert(start);

    let mut res = None;
    let mut count = 0;
    'outer: while !todo.is_empty() {
        count = count + 1;
        // println!("Count: {} : {:?}", count, todo);
        let mut next_todo: HashSet<Point> = HashSet::new();
        for pos in todo {
            for dir in Direction::VALUES {
                if let Some(np) = grid.point_in_direction(pos, dir) {
                    if np == end {
                        res = Some(count);
                        break 'outer;
                    }
                    if visited.insert(np) {
                        let nv = grid.get_point(np).unwrap();
                        if nv == '.' {
                            next_todo.insert(np);
                        }
                    }
                }
            }
        }
        todo = next_todo;
    }
    res
}

pub fn solve() -> String {
    let contents = fs::read_to_string("data/day18/input.txt").unwrap();
    // let contents = fs::read_to_string("data/day18/ex.txt").unwrap();
    let positions: Vec<(usize, usize)> = contents
        .split("\n")
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut iter = s.split(",");
            let x = iter.next().clone().unwrap().parse().unwrap();
            let y = iter.next().clone().unwrap().parse().unwrap();
            (x, y)
        })
        .collect();

    format!(
        "Task1: {}\nTask2: {}",
        // task1(&positions, 7, 7, 12),
        task1(&positions, 71, 71, 1024),
        // task2(&positions, 7, 7, 12),
        task2(&positions, 71, 71, 1024),
    )
}
