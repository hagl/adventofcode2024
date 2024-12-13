use std::collections::HashSet;
use std::fs;

use crate::grid::{Direction, Grid, Point};

fn task1(map: &Grid) -> usize {
    let mut processed: HashSet<Point> = HashSet::new();
    map.iter()
        .map(|start| {
            if processed.insert(start) {
                let mut local_processed: HashSet<Point> = HashSet::new();
                let (area, border) =
                    find_area(map, start, &mut local_processed, &mut HashSet::new());
                processed.extend(&local_processed);
                area * border
            } else {
                0
            }
        })
        .sum()
}

fn task2(map: &Grid) -> usize {
    let mut processed: HashSet<Point> = HashSet::new();
    map.iter()
        .map(|start| {
            if processed.insert(start) {
                let mut borders: HashSet<(Point, Direction)> = HashSet::new();
                let mut local_processed: HashSet<Point> = HashSet::new();
                let (area, _) = find_area(map, start, &mut local_processed, &mut borders);
                processed.extend(&local_processed);

                let border_count = find_borders(map, &mut borders);
                area * border_count
            } else {
                0
            }
        })
        .sum()
}

fn extend_border(
    map: &Grid,
    p: Point,
    border_dir: Direction,
    extend_dir: Direction,
    borders: &mut HashSet<(Point, Direction)>,
) {
    let mut np = p;
    loop {
        if let Some(left) = map.point_in_direction(np, extend_dir) {
            np = left;
            if !borders.remove(&(np, border_dir)) {
                break;
            }
        } else {
            break;
        }
    }
}

fn find_borders(map: &Grid, borders: &mut HashSet<(Point, Direction)>) -> usize {
    let mut acc = 0;
    while let Some((p, d)) = borders.iter().cloned().next() {
        acc += 1;
        borders.remove(&(p, d));
        if d == Direction::Up || d == Direction::Down {
            extend_border(map, p, d, Direction::Left, borders);
            extend_border(map, p, d, Direction::Right, borders);
        } else {
            extend_border(map, p, d, Direction::Up, borders);
            extend_border(map, p, d, Direction::Down, borders);
        }
    }
    acc
}

fn find_area(
    map: &Grid,
    p: Point,
    local_processed: &mut HashSet<Point>,
    borders: &mut HashSet<(Point, Direction)>,
) -> (usize, usize) {
    let mut area = 1;
    let mut border = 0;
    local_processed.insert(p);
    for d in Direction::VALUES {
        if let Some(np) = map.point_in_direction(p, d) {
            if !local_processed.contains(&np) {
                if map.get_point(p) == map.get_point(np) {
                    let (na, nb) = find_area(map, np, local_processed, borders);
                    area += na;
                    border += nb;
                } else {
                    border += 1;
                    borders.insert((p, d));
                }
            }
        } else {
            border += 1;
            borders.insert((p, d));
        }
    }
    (area, border)
}

pub fn solve() -> String {
    let contents = fs::read_to_string("data/day12/input.txt").unwrap();
    // let contents = fs::read_to_string("data/day12/ex1.txt").unwrap();

    let grid: Grid = Grid::from_str(&contents);
    format!("Task1: {},\nTask2: {}", task1(&grid), task2(&grid))
}
