use std::collections::HashMap;
use std::fs;

use num::{BigInt, FromPrimitive};

fn task1(stones: &Vec<i64>) -> usize {
    let mut cache: HashMap<(BigInt, usize), usize> = HashMap::new();
    stones
        .iter()
        .map(|n| count(BigInt::from_i64(*n).unwrap(), 25, &mut cache))
        .sum()
}
fn task2(stones: &Vec<i64>) -> usize {
    let mut cache: HashMap<(BigInt, usize), usize> = HashMap::new();
    stones
        .iter()
        .map(|n| count(BigInt::from_i64(*n).unwrap(), 75, &mut cache))
        .sum()
}

fn count(b: BigInt, steps: usize, cache: &mut HashMap<(BigInt, usize), usize>) -> usize {
    if let Some(v) = cache.get(&(b.clone(), steps)) {
        *v
    } else {
        if steps == 0 {
            1
        } else {
            let res = if b == BigInt::ZERO {
                count(BigInt::from(1), steps - 1, cache)
            } else {
                let digits = b.to_string();
                if digits.len() % 2 == 0 {
                    count(
                        digits[0..(digits.len() / 2)].parse().unwrap(),
                        steps - 1,
                        cache,
                    ) + count(
                        digits[(digits.len() / 2)..].parse().unwrap(),
                        steps - 1,
                        cache,
                    )
                } else {
                    count(b.clone() * 2024, steps - 1, cache)
                }
            };
            cache.insert((b.clone(), steps), res);
            res
        }
    }
}

pub fn solve() -> String {
    let contents = fs::read_to_string("data/day11/input.txt").unwrap();
    // fs::read_to_string("data/day11/ex.txt").unwrap();

    let stones: Vec<i64> = contents
        .split([' ', '\n'])
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    format!("Task1: {},\nTask2: {}", task1(&stones), task2(&stones))
}
