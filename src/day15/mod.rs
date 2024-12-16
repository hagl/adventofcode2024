use std::collections::HashSet;
use std::fs;

use crate::grid::{Direction, Grid, Point};

fn task1(map: &mut Grid, moves: &str) -> usize {
    let mut pos = map
        .iter()
        .find(|p| map.get_point(*p).unwrap() == '@')
        .unwrap();
    map.set_point(pos, '.');
    for c in moves.chars() {
        let dir = direction(c);
        let next_pos = map.point_in_direction(pos, dir).unwrap();
        let mut next_empty = next_pos;
        while map.get_point(next_empty).unwrap() == 'O' {
            next_empty = map.point_in_direction(next_empty, dir).unwrap();
        }
        if map.get_point(next_empty).unwrap() == '.' {
            map.set_point(next_empty, 'O');
            map.set_point(next_pos, '.');
            pos = next_pos;
        } else if map.get_point(next_empty).unwrap() == '#' {
            continue;
        } else {
            // panic!(format!("Unexpected empty  {:?}", map.get_point(next_empty)));
            panic!("Unexpected empty");
        }
    }
    calculate(map)
}

fn direction(c: char) -> Direction {
    match c {
        '<' => Direction::Left,
        '^' => Direction::Up,
        '>' => Direction::Right,
        'v' => Direction::Down,
        _ => todo!(),
    }
}

fn calculate(map: &Grid) -> usize {
    map.iter()
        .map(|p| {
            if map.get_point(p).unwrap() == 'O' {
                100 * p.y + p.x
            } else {
                0
            }
        })
        .sum()
}

fn calculate2(map: &Grid) -> usize {
    map.iter()
        .map(|p| {
            if map.get_point(p).unwrap() == '[' {
                100 * p.y + p.x
            } else {
                0
            }
        })
        .sum()
}

fn task2(map: &mut Grid, moves: &str) -> usize {
    let mut pos = map
        .iter()
        .find(|p| map.get_point(*p).unwrap() == '@')
        .unwrap();
    map.set_point(pos, '.');
    println!("Map: \n{}\n\n", map.to_string());

    for c in moves.chars() {
        let dir = direction(c);
        let mut changes: Vec<(Point, char)> = vec![(pos, '.')];
        let next_pos = map.point_in_direction(pos, dir).unwrap();
        if collectChanges(map, &mut changes, next_pos, dir) {
            map.set_point(next_pos, '.');
            for (p, c) in changes {
                map.set_point(p, c);
            }
            pos = next_pos;
        }
        // println!("Move {} \n{}\n\n", c, map.to_string2(pos, '@'));
    }
    calculate2(map)
}

fn collectChanges(
    map: &Grid,
    changes: &mut Vec<(Point, char)>,
    pos: Point,
    dir: Direction,
) -> bool {
    match map.get_point(pos).unwrap() {
        '.' => true,
        '#' => false,
        '[' => {
            let next_pos = map.point_in_direction(pos, dir).unwrap();
            changes.push((next_pos, '['));
            if dir == Direction::Left || dir == Direction::Right {
                collectChanges(map, changes, next_pos, dir)
            } else {
                // move other part of the box too
                let right_pos = map.point_in_direction(pos, Direction::Right).unwrap();
                // changes.push((right_pos, '.'));
                changes.insert(0, (right_pos, '.'));
                let next_right_pos = map.point_in_direction(right_pos, dir).unwrap();
                changes.push((next_right_pos, ']'));
                collectChanges(map, changes, next_pos, dir)
                    && collectChanges(map, changes, next_right_pos, dir)
            }
        }
        ']' => {
            let next_pos = map.point_in_direction(pos, dir).unwrap();
            changes.push((next_pos, ']'));
            if (dir == Direction::Left || dir == Direction::Right) {
                collectChanges(map, changes, next_pos, dir)
            } else {
                // move other part of the box too
                let left_pos = map.point_in_direction(pos, Direction::Left).unwrap();
                changes.insert(0, (left_pos, '.'));
                let next_left_pos = map.point_in_direction(left_pos, dir).unwrap();
                changes.push((next_left_pos, '['));
                collectChanges(map, changes, next_pos, dir)
                    && collectChanges(map, changes, next_left_pos, dir)
            }
        }
        c => {
            println!("c = {}", c);
            todo!()
        }
    }
}

pub fn solve() -> String {
    let contents = fs::read_to_string("data/day15/input.txt").unwrap();
    // let contents = fs::read_to_string("data/day15/ex2.txt").unwrap();
    // let contents = fs::read_to_string("data/day15/ex1.txt").unwrap();
    // let contents = fs::read_to_string("data/day15/ex.txt").unwrap();

    let mut iter = contents.split("\n\n");
    let grid = iter.next().unwrap();

    let grid2 = grid
        .to_string()
        .replace('#', "##")
        .replace('O', "[]")
        .replace('.', "..")
        .replace('@', "@.");

    let mut grid = Grid::from_str(grid);
    let mut grid2 = Grid::from_str(&grid2);

    let moves = iter
        .next()
        .unwrap()
        .split("\n")
        .fold("".to_string(), |mut acc, s| {
            acc.push_str(s);
            acc
        });

    println!("{:?} \n\n {}", grid2, moves);
    format!(
        "Task1: {},\nTask2: {}",
        task1(&mut grid, &moves),
        task2(&mut grid2, &moves)
    )
}
