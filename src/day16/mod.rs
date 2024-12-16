use std::collections::{HashMap, HashSet};
use std::fs;

use crate::grid::{self, Direction, Grid, Point};

fn task1(map: &mut Grid) -> usize {
    let mut minimums: HashMap<(Point, Direction), usize> = HashMap::new();
    let start = map
        .iter()
        .find(|p| map.get_point(*p).unwrap() == 'S')
        .unwrap();
    let end = map
        .iter()
        .find(|p| map.get_point(*p).unwrap() == 'E')
        .unwrap();
    minimums.insert((start, Direction::Right), 0);
    let mut todo: Vec<(Point, Direction)> = vec![(start, Direction::Right)];
    while let Some((pos, dir)) = todo.pop() {
        let score = *minimums.get(&(pos, dir)).unwrap();
        if let Some(new_pos) = map.point_in_direction(pos, dir) {
            if map.get_point(new_pos).unwrap() != '#' {
                // can move
                insert_if_less(&mut minimums, &mut todo, new_pos, dir, score + 1);
            }
        }
        insert_if_less(&mut minimums, &mut todo, pos, dir.left(), score + 1000);
        insert_if_less(
            &mut minimums,
            &mut todo,
            pos,
            dir.left().left(),
            score + 2000,
        );
        insert_if_less(&mut minimums, &mut todo, pos, dir.right(), score + 1000);
    }

    *Direction::VALUES
        .iter()
        .filter_map(|dir| minimums.get(&(end, *dir)))
        .min()
        .unwrap()
}

fn insert_if_less(
    minimums: &mut HashMap<(Point, Direction), usize>,
    todo: &mut Vec<(Point, Direction)>,
    pos: Point,
    dir: Direction,
    score: usize,
) {
    if let Some(previous) = minimums.get(&(pos, dir)) {
        if score < *previous {
            minimums.insert((pos, dir), score);
            // if (!todo.contains(&(pos, dir))) {
            todo.push((pos, dir));
            // }
        }
    } else {
        minimums.insert((pos, dir), score);
        // if (!todo.contains(&(pos, dir))) {
        todo.push((pos, dir));
        // }
    }
}

pub fn solve() -> String {
    let contents = fs::read_to_string("data/day16/input.txt").unwrap();
    // let contents = fs::read_to_string("data/day16/ex.txt").unwrap();
    // let contents = fs::read_to_string("data/day16/ex2.txt").unwrap();
    // let contents = fs::read_to_string("data/day16/ex1.txt").unwrap();

    let mut grid = Grid::from_str(&contents);

    format!(
        "Task1: {},\nTask2: {}",
        task1(&mut grid),
        "task2(&mut grid2, &moves)"
    )
}
