use std::collections::HashSet;
use std::fs;

use crate::grid::{Direction, Grid, Point};

fn task1(map: &Grid) -> usize {
    let mut todo: Vec<Point> = vec![Point { x: 0, y: 0 }];
    let mut processed: HashSet<Point> = HashSet::new();
    let mut acc = 0;
    while let Some(start) = todo.pop() {
        let value = map.get_point(start).unwrap();
        if processed.insert(start) {
            let (area, border) = find_area(
                map,
                start,
                value,
                &mut todo,
                &mut processed,
                &mut HashSet::new(),
                &mut HashSet::new(),
            );
            acc += area * border;
        }
    }
    acc
}

fn task2(map: &Grid) -> usize {
    let mut todo: Vec<Point> = vec![Point { x: 0, y: 0 }];
    let mut processed: HashSet<Point> = HashSet::new();
    let mut acc = 0;
    while let Some(start) = todo.pop() {
        let value = map.get_point(start).unwrap();
        let mut borders: HashSet<(Point, Direction)> = HashSet::new();
        if processed.insert(start) {
            let (area, _) = find_area(
                map,
                start,
                value,
                &mut todo,
                &mut processed,
                &mut HashSet::new(),
                &mut borders,
            );
            let border_count = find_borders(map, &mut borders);
            acc += area * border_count;
        }
    }
    acc
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
    value: char,
    todo: &mut Vec<Point>,
    processed: &mut HashSet<Point>,
    local_processed: &mut HashSet<Point>,
    borders: &mut HashSet<(Point, Direction)>,
) -> (usize, usize) {
    let mut area = 1;
    let mut border = 0;
    processed.insert(p);
    local_processed.insert(p);
    for d in Direction::VALUES {
        if let Some(np) = map.point_in_direction(p, d) {
            if !local_processed.contains(&np) {
                if let Some(nv) = map.get_point(np) {
                    if nv == value {
                        let (na, nb) =
                            find_area(map, np, nv, todo, processed, local_processed, borders);
                        area += na;
                        border += nb;
                    } else {
                        todo.push(np);
                        border += 1;
                        borders.insert((p, d));
                    }
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
