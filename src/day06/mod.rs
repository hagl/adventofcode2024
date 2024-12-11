use std::collections::HashSet;
use std::fs;

use crate::grid::{Direction, Grid};

fn find_start(input: &Grid) -> (usize, usize) {
    let mut p = (0, 0);

    for x in 0..input.width {
        for y in 0..input.height {
            if input.get((x, y)).unwrap() == '^' {
                p = (x, y);
            }
        }
    }
    p
}

fn task1(input: &Grid) -> usize {
    // find start position
    let mut p = find_start(input);

    let mut visited = HashSet::new();
    visited.insert(p);
    let mut dir = Direction::Up;
    loop {
        if let Some(np) = input.pos_in_direction(p, &dir) {
            if input.get(np).unwrap() == '#' {
                dir = dir.right();
            } else {
                p = np;
                visited.insert(p);
            }
        } else {
            break;
        }
    }
    visited.len()
}

fn task2(input: &Grid) -> usize {
    let mut pos = find_start(input);
    let mut dir = Direction::Up;

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut visited_dir: HashSet<((usize, usize), Direction)> = HashSet::new();
    visited.insert(pos);
    visited_dir.insert((pos, dir));

    let mut count = 0;

    loop {
        if let Some(np) = input.pos_in_direction(pos, &dir) {
            if input.get(np).unwrap() == '#' {
                dir = dir.right();
            } else {
                if visited.insert(np)
                    && check_loop(input, pos, dir.right(), np, visited_dir.clone())
                {
                    count += 1;
                }
                visited_dir.insert((np, dir));
                pos = np;
            }
        } else {
            break;
        }
    }
    count
}

fn check_loop(
    input: &Grid,
    mut pos: (usize, usize),
    mut dir: Direction,
    blocked: (usize, usize),
    mut visited: HashSet<((usize, usize), Direction)>,
) -> bool {
    visited.insert((pos, dir));
    loop {
        if let Some(np) = input.pos_in_direction(pos, &dir) {
            if input.get(np).unwrap() == '#' || np == blocked {
                dir = dir.right();
            } else {
                pos = np;
                if !visited.insert((pos, dir)) {
                    break true;
                }
            }
        } else {
            break false;
        }
    }
}

pub fn solve() -> String {
    let contents = fs::read_to_string("data/day06/input.txt").unwrap();
    // fs::read_to_string("data/day06/ex.txt").unwrap();
    let grid = Grid::from_str(&contents);
    format!("Task1: {},\nTask2: {}", task1(&grid), task2(&grid))
}
