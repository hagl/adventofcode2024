use std::collections::HashSet;
use std::fs;
#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn get(input: &Vec<Vec<char>>, x: i32, y: i32) -> Option<&char> {
    if x < 0 || y < 0 {
        None
    } else {
        let line = input.get(usize::try_from(y).ok()?)?;
        line.get(usize::try_from(x).ok()?)
    }
}

fn check(input: &Vec<Vec<char>>, x: i32, y: i32, dx: i32, dy: i32) -> bool {
    let f = || -> Option<bool> {
        Some(
            *(get(input, x, y)?) == 'X'
                && *(get(input, x + dx, y + dy)?) == 'M'
                && *(get(input, x + dx + dx, y + dy + dy)?) == 'A'
                && *(get(input, x + dx + dx + dx, y + dy + dy + dy)?) == 'S',
        )
    };
    let res = f().unwrap_or(false);
    // println!("{},{}  {},{} -> {}", x, y, dx, dy, res);
    res
}

fn task1(input: &Vec<Vec<char>>) -> usize {
    let height: i32 = input.len().try_into().unwrap();
    let width: i32 = input[0].len().try_into().unwrap();
    // find start position
    let mut sx: i32 = -1;
    let mut sy: i32 = -1;

    for x in 0..width {
        for y in 0..height {
            if *(get(input, x, y).unwrap()) == '^' {
                sx = x;
                sy = y;
            }
        }
    }
    let mut dir = Direction::Up;
    let mut visited = HashSet::new();
    visited.insert((sx, sy));

    loop {
        let (nx, ny) = nextPos(sx, sy, &dir);
        match get(input, nx, ny) {
            None => break,
            Some(c) => {
                if *c == '#' {
                    dir = turnRight(dir);
                } else {
                    sx = nx;
                    sy = ny;
                    visited.insert((sx, sy));
                }
            }
        }
    }

    visited.len()
}

fn task2(input: &Vec<Vec<char>>) -> usize {
    let height: i32 = input.len().try_into().unwrap();
    let width: i32 = input[0].len().try_into().unwrap();
    // find start position
    let mut sx: i32 = -1;
    let mut sy: i32 = -1;

    for x in 0..width {
        for y in 0..height {
            if *(get(input, x, y).unwrap()) == '^' {
                sx = x;
                sy = y;
            }
        }
    }
    let mut count = 0;
    let mut dir = Direction::Up;
    let mut visited = HashSet::new();
    visited.insert((sx, sy));

    let mut cx = sx;
    let mut cy = sy;
    loop {
        let (nx, ny) = nextPos(cx, cy, &dir);
        match get(input, nx, ny) {
            None => break,
            Some(c) => {
                if *c == '#' {
                    dir = turnRight(dir);
                } else {
                    cx = nx;
                    cy = ny;
                    visited.insert((cx, cy));
                }
            }
        }
    }

    let mut count = 0;
    for (px, py) in visited {
        if (px != sx || py != sy) {
            // don't block starting position
            if checkLoop(input, sx, sy, px, py) {
                count += 1;
            }
        }
    }
    count
}

fn checkLoop(input: &Vec<Vec<char>>, sx: i32, sy: i32, px: i32, py: i32) -> bool {
    let mut dir = Direction::Up;
    let mut visited = HashSet::new();
    visited.insert((sx, sy, dir));
    let mut cx = sx;
    let mut cy = sy;
    loop {
        let (nx, ny) = nextPos(cx, cy, &dir);
        match get(input, nx, ny) {
            None => break false,
            Some(c) => {
                if *c == '#' || nx == px && ny == py {
                    dir = turnRight(dir);
                } else {
                    cx = nx;
                    cy = ny;
                    if (!visited.insert((cx, cy, dir))) {
                        break true;
                    }
                }
            }
        }
    }
}

pub fn nextPos(x: i32, y: i32, dir: &Direction) -> (i32, i32) {
    match dir {
        Direction::Up => (x, y - 1),
        Direction::Right => (x + 1, y),
        Direction::Down => (x, y + 1),
        Direction::Left => (x - 1, y),
    }
}

pub fn turnRight(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

pub fn solve() -> String {
    let contents =
        fs::read_to_string("data/day06/input.txt").expect("Should have been able to read the file");
    // fs::read_to_string("data/day06/ex.txt").expect("Should have been able to read the file");
    let array: Vec<Vec<char>> = contents
        .split("\n")
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect())
        .collect();
    format!("Task1: {},\nTask2: {}", task1(&array), task2(&array),)
}
