use core::cmp::Ordering::*;
use std::fs;

fn task1(rules: &[String], updates: &[String]) -> (i64, i64) {
    let mut result = 0;
    let mut result2 = 0;
    for u in updates {
        if !u.is_empty() {
            let values: Vec<String> = u.split(",").map(|s| s.to_string()).collect();
            let len = values.len();
            let mut valid = true;
            for i in 0..(len - 1) {
                for j in (i + 1)..len {
                    let check = format!("{}|{}", values[j], values[i]);
                    if rules.contains(&check) {
                        valid = false;
                    }
                }
            }
            if valid {
                let mid = values[len / 2].parse::<i64>().unwrap();
                result += mid;
            } else {
                let sorted = sort_line(rules, &values);
                let mid = sorted[len / 2].parse::<i64>().unwrap();
                result2 += mid;
            }
        }
    }
    (result, result2)
}

fn sort_line(rules: &[String], values: &Vec<String>) -> Vec<String> {
    let mut res = values.clone();
    res.sort_by(|a, b| {
        if rules.contains(&format!("{}|{}", a, b)) {
            Less
        } else if rules.contains(&format!("{}|{}", b, a)) {
            Greater
        } else {
            Equal
        }
    });
    res
}

pub fn solve() -> String {
    let contents =
        // fs::read_to_string("data/day05/ex.txt").unwrap();
    fs::read_to_string("data/day05/input.txt").unwrap();

    let lines: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();
    let index = lines.iter().position(|r| r.is_empty()).unwrap();
    let (rules, updates) = lines.split_at(index);
    let (t1, t2) = task1(rules, updates);
    format!("Task1: {},\nTask2: {}", t1, t2)
}
