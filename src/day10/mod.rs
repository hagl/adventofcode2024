use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

use crate::grid::{Direction, Grid};

fn task1(map: &Grid) -> usize {
    get_trail_heads(map)
        .iter()
        .map(|t| get_trail_ends(map, t).len())
        .sum()
}

fn get_trail_heads(map: &Grid) -> HashSet<(usize, usize)> {
    let mut trailheads: HashSet<(usize, usize)> = HashSet::new();
    for x in 0..map.width {
        for y in 0..map.height {
            if map.get((x, y)) == Some('0') {
                trailheads.insert((x, y));
            }
        }
    }
    trailheads
}

fn get_trail_ends(map: &Grid, trailhead: &(usize, usize)) -> HashSet<(usize, usize)> {
    let mut positions: HashSet<(usize, usize)> = HashSet::new();
    positions.insert(*trailhead);
    for i in '1'..='9' {
        let mut new_positions: HashSet<(usize, usize)> = HashSet::new();
        for (x, y) in positions {
            for d in Direction::VALUES {
                if let Some(pos) = map.pos_in_direction((x, y), &d) {
                    if map.get(pos) == Some(i) {
                        new_positions.insert(pos);
                    }
                }
            }
        }
        positions = new_positions;
    }
    positions
}

fn task2(map: &Grid) -> usize {
    let trailheads = get_trail_heads(map);

    let mut acc = 0;
    for t in trailheads {
        let positions = get_trail_ends_count(map, t);
        acc += positions.values().sum::<usize>();
    }
    acc
}

fn get_trail_ends_count(map: &Grid, trailhead: (usize, usize)) -> HashMap<(usize, usize), usize> {
    let mut positions: HashMap<(usize, usize), usize> = HashMap::new();
    positions.insert(trailhead, 1);
    for i in '1'..='9' {
        let mut new_positions: HashMap<(usize, usize), usize> = HashMap::new();
        for ((x, y), count) in positions {
            for d in Direction::VALUES {
                if let Some(pos) = map.pos_in_direction((x, y), &d) {
                    if map.get(pos) == Some(i) {
                        let entry = new_positions.entry(pos).or_insert(0);
                        *entry += count;
                    }
                }
            }
        }
        positions = new_positions;
    }
    positions
}

pub fn solve() -> String {
    let contents =
        fs::read_to_string("data/day10/input.txt").unwrap();
    // fs::read_to_string("data/day10/ex1.txt").unwrap();

    let grid: Grid = Grid::from_str(&contents);
    format!("Task1: {},\nTask2: {}", task1(&grid), task2(&grid))
}
