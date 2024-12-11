use regex::Regex;
use std::{fs, num::ParseIntError};

fn task1(input: &str) -> Result<i64, ParseIntError> {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let mut prod: i64 = 0;
    for (_, [f1, f2]) in re.captures_iter(input).map(|caps| caps.extract()) {
        prod = prod + f1.parse::<i64>()? * f2.parse::<i64>()?;
    }
    Ok(prod)
}

fn task2(input: &str) -> Result<i64, ParseIntError> {
    let re = Regex::new(r"don't\(\)|do\(\)").unwrap();
    let mut prod: i64 = 0;
    let mut enabled = true;
    let mut last = 0;

    for m in re.find_iter(input) {
        let index = m.start();
        let matched = m.as_str();
        let segment = &input[last..index];
        if enabled {
            prod = prod + task1(segment).unwrap();
        }
        enabled = matched == "do()";

        last = m.end();
    }
    if last < input.len() {
        if enabled {
            prod = prod + task1(&input[last..]).unwrap();
        }
    }
    Ok(prod)
}

pub fn solve() -> String {
    let contents =
        fs::read_to_string("data/day03/input.txt").unwrap();

    let prod = task1(&contents).unwrap();
    let prod2 = task2(&contents).unwrap();
    format!("Task1: {},\nTask2: {}", prod, prod2)
}
