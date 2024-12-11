use std::collections::HashSet;
use std::fs;
#[derive(PartialEq, Eq, Hash, Clone)]
struct Line {
    res: i64,
    numbers: Vec<i64>,
}

fn get(input: &Vec<Vec<char>>, x: i32, y: i32) -> Option<&char> {
    if x < 0 || y < 0 {
        None
    } else {
        let line = input.get(usize::try_from(y).ok()?)?;
        line.get(usize::try_from(x).ok()?)
    }
}

fn task1(lines: &Vec<Line>) -> i64 {
    let mut count = 0;
    for line in lines {
        let len = line.numbers.len();
        let ops = len - 1;
        let base: usize = 2;
        let combinations = base.pow(ops.try_into().unwrap());
        for c in 0..combinations {
            let mut r = line.numbers[0];
            let mut bit = 1;
            for o in 0..ops {
                r = if bit & c != 0 {
                    r * line.numbers[o + 1]
                } else {
                    r + line.numbers[o + 1]
                };
                bit = bit << 1
            }
            if r == line.res {
                count += line.res;
                break;
            }
        }
    }
    count
}

fn check(res: i64, acc: i64, numbers: &[i64]) -> bool {
    if numbers.len() == 0 {
        res == acc
    } else {
        let n = numbers[0];
        check(res, format!("{}{}", acc, n).parse().unwrap(), &numbers[1..])
            || check(res, acc + n, &numbers[1..])
            || check(res, acc * n, &numbers[1..])
    }
}

fn task2(lines: &Vec<Line>) -> i64 {
    let mut result = 0;
    for line in lines {
        if check(line.res, line.numbers[0], &line.numbers[1..]) {
            result += line.res
        }
    }
    result
}

pub fn solve() -> String {
    let contents =
        fs::read_to_string("data/day07/input.txt").unwrap();
    // fs::read_to_string("data/day07/ex.txt").unwrap();
    let lines: Vec<Line> = contents
        .split("\n")
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.split([' ', ':'])
                .map(|s| s.to_string())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<i64>().unwrap())
                .collect()
        })
        .map(|vec: Vec<i64>| Line {
            res: vec[0],
            numbers: vec[1..].to_vec(),
        })
        .collect();
    format!("Task1: {},\nTask2: {}", task1(&lines), task2(&lines),)
}
