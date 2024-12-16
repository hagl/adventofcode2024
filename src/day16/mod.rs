use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::Hash;

use crate::grid::{Direction, Grid, Point};

fn pop<T>(set: &mut HashSet<T>) -> Option<T>
where
    T: Hash + Eq + Clone,
{
    if let Some(el) = set.iter().next().cloned() {
        set.remove(&el);
        Some(el)
    } else {
        None
    }
}

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
    // let mut todo: Vec<(Point, Direction)> = vec![(start, Direction::Right)];
    let mut todo: HashSet<(Point, Direction)> = HashSet::new();
    todo.insert((start, Direction::Right));
    // while let Some((pos, dir)) = todo.pop() {
    while let Some((pos, dir)) = pop(&mut todo) {
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

    let minimum = *Direction::VALUES
        .iter()
        .filter_map(|dir| minimums.get(&(end, *dir)))
        .min()
        .unwrap();

    minimum
}

fn task2(map: &mut Grid) -> usize {
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
    let mut todo: HashSet<(Point, Direction)> = HashSet::new();
    todo.insert((start, Direction::Right));
    while let Some((pos, dir)) = pop(&mut todo) {
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

    let minimum = *Direction::VALUES
        .iter()
        .filter_map(|dir| minimums.get(&(end, *dir)))
        .min()
        .unwrap();

    let mut on_path: HashSet<Point> = HashSet::new();
    let mut todo: HashSet<(Point, Direction, usize)> = Direction::VALUES
        .iter()
        .filter(|dir| *minimums.get(&(end, **dir)).unwrap() == minimum)
        .map(|dir| (end, *dir, minimum))
        .collect();
    on_path.insert(end);
    while let Some((pos, dir, score)) = pop(&mut todo) {
        if let Some(prev_pos) = map.point_in_direction(pos, dir.left().left()) {
            is_on_path(&minimums, &mut on_path, &mut todo, prev_pos, dir, score - 1);
        }
        is_on_path(
            &minimums,
            &mut on_path,
            &mut todo,
            pos,
            dir.left(),
            score - 1000,
        );
        is_on_path(
            &minimums,
            &mut on_path,
            &mut todo,
            pos,
            dir.left().left(),
            score - 2000,
        );
        is_on_path(
            &minimums,
            &mut on_path,
            &mut todo,
            pos,
            dir.right(),
            score - 1000,
        );
    }
    on_path.len()
}

fn is_on_path(
    minimums: &HashMap<(Point, Direction), usize>,
    on_path: &mut HashSet<Point>,
    todo: &mut HashSet<(Point, Direction, usize)>,
    pos: Point,
    dir: Direction,
    score: usize,
) {
    if let Some(s) = minimums.get(&(pos, dir)) {
        if *s == score {
            on_path.insert(pos);
            todo.insert((pos, dir, score));
        }
    }
}

fn insert_if_less(
    minimums: &mut HashMap<(Point, Direction), usize>,
    todo: &mut HashSet<(Point, Direction)>,
    pos: Point,
    dir: Direction,
    score: usize,
) {
    if let Some(previous) = minimums.get(&(pos, dir)) {
        if score < *previous {
            minimums.insert((pos, dir), score);
            todo.insert((pos, dir));
        }
    } else {
        minimums.insert((pos, dir), score);
        todo.insert((pos, dir));
    }
}

pub fn solve() -> String {
    let contents = fs::read_to_string("data/day16/input.txt").unwrap();
    // let contents = fs::read_to_string("data/day16/ex.txt").unwrap();

    let mut grid = Grid::from_str(&contents);

    format!("Task1: {},\nTask2: {}", task1(&mut grid), task2(&mut grid))
}
