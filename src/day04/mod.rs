use std::fs;

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

fn task1(input: &Vec<Vec<char>>) -> i64 {
    let height = input.len();
    let width = input[0].len();
    let mut count = 0;
    for x in 0..width {
        for y in 0..height {
            for dx in -1..2 {
                for dy in -1..2 {
                    if check(input, x.try_into().unwrap(), y.try_into().unwrap(), dx, dy) {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

fn check2(input: &Vec<Vec<char>>, x: i32, y: i32, d1: bool, d2: bool) -> bool {
    let f = || -> Option<bool> {
        Some(
            *(get(input, x, y)?) == 'A'
                && *(get(input, x - 1, y - 1)?) == if d1 { 'M' } else { 'S' }
                && *(get(input, x + 1, y + 1)?) == if d1 { 'S' } else { 'M' }
                && *(get(input, x - 1, y + 1)?) == if d2 { 'M' } else { 'S' }
                && *(get(input, x + 1, y - 1)?) == if d2 { 'S' } else { 'M' },
        )
    };
    let res = f().unwrap_or(false);
    // println!("{},{}  {},{} -> {}", x, y, dx, dy, res);
    res
}

fn task2(input: &Vec<Vec<char>>) -> i64 {
    let height = input.len();
    let width = input[0].len();
    let mut count = 0;
    for x in 0..width {
        for y in 0..height {
            for d1 in [false, true] {
                for d2 in [false, true] {
                    if check2(input, x.try_into().unwrap(), y.try_into().unwrap(), d1, d2) {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

pub fn solve() -> String {
    let contents =
        fs::read_to_string("data/day04/input.txt").unwrap();
    // fs::read_to_string("data/day04/ex.txt").unwrap();
    let array: Vec<Vec<char>> = contents
        .split("\n")
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect())
        .collect();
    format!("Task1: {},\nTask2: {}", task1(&array), task2(&array),)
}
