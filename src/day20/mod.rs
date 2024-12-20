use std::collections::{HashMap, HashSet};
use std::fs;

use crate::grid::{Direction, Grid, Point};

fn run(grid: &Grid, start: &Point, end: &Point) -> HashMap<Point, usize> {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut todo: HashSet<Point> = HashSet::new();
    let mut result: HashMap<Point, usize> = HashMap::new();

    let mut count = 0;

    visited.insert(*start);
    todo.insert(*start);
    result.insert(*start, count);

    'outer: while !todo.is_empty() {
        count = count + 1;
        let mut next_todo: HashSet<Point> = HashSet::new();
        for pos in todo {
            for dir in Direction::VALUES {
                if let Some(np) = grid.point_in_direction(pos, dir) {
                    if np == *end {
                        result.insert(np, count);
                        break 'outer;
                    }
                    if visited.insert(np) {
                        let nv = grid.get_point(np).unwrap();
                        if nv == '.' {
                            next_todo.insert(np);
                            result.insert(np, count);
                        }
                    }
                }
            }
        }
        todo = next_todo;
    }
    result
}

fn task1(map: &mut Grid) -> usize {
    task(map, 2)
}

fn task2(map: &mut Grid) -> usize {
    task(map, 20)
}

fn task(map: &mut Grid, cheat_count: i32) -> usize {
    let start = map
        .iter()
        .find(|p| map.get_point(*p).unwrap() == 'S')
        .unwrap();
    let end = map
        .iter()
        .find(|p| map.get_point(*p).unwrap() == 'E')
        .unwrap();

    let start_costs = run(&map, &start, &end);
    let end_costs = run(&map, &end, &start);

    let baseline = start_costs.get(&end).unwrap();

    let set: HashSet<usize> = map
        .iter()
        .map(|cheat_start| {
            if let Some(start_cost) = start_costs.get(&cheat_start) {
                if let Some(end_cost) = end_costs.get(&cheat_start) {
                    start_cost + end_cost
                } else {
                    0
                }
            } else {
                0
            }
        })
        .collect();

    let result = map
        .iter()
        .map(|cheat_start| {
            let mut result = 0;
            if let Some(start_cost) = start_costs.get(&cheat_start) {
                for x in (-cheat_count)..=cheat_count {
                    let d = cheat_count - i32::abs(x);
                    for y in -d..=d {
                        if let Some(cheat_end) = map.move_point(cheat_start, x, y) {
                            if let Some(end_cost) = end_costs.get(&cheat_end) {
                                let cost = start_cost
                                    + end_cost
                                    + cheat_start.manhatten_distance(&cheat_end);
                                if cost < baseline - 99 {
                                    result = result + 1;
                                }
                            }
                        }
                    }
                }
            }
            result
        })
        .sum();
    result
}

pub fn solve() -> String {
    let contents = fs::read_to_string("data/day20/input.txt").unwrap();
    // let contents = fs::read_to_string("data/day20/ex.txt").unwrap();

    let mut grid = Grid::from_str(&contents);

    format!("Task1: {},\nTask2: {}", task1(&mut grid), task2(&mut grid))
}
