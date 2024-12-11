use std::collections::HashMap;
use std::fs;

fn task1(stones: &Vec<i64>) -> usize {
    let mut cache: HashMap<(i64, usize), usize> = HashMap::new();
    stones.iter().map(|n| count(*n, 25, &mut cache)).sum()
}
fn task2(stones: &Vec<i64>) -> usize {
    let mut cache: HashMap<(i64, usize), usize> = HashMap::new();
    stones.iter().map(|n| count(*n, 75, &mut cache)).sum()
}

fn num_digits(n: i64) -> u32 {
    let mut n = n;
    let mut c = 0;
    while n != 0 {
        n = n / 10;
        c += 1;
    }
    c
}

fn count(b: i64, steps: usize, cache: &mut HashMap<(i64, usize), usize>) -> usize {
    if let Some(v) = cache.get(&(b, steps)) {
        *v
    } else {
        if steps == 0 {
            1
        } else {
            let res = if b == 0 {
                count(1, steps - 1, cache)
            } else {
                let nd = num_digits(b);
                if nd % 2 == 0 {
                    let base: i64 = 10;
                    let d = base.pow(nd / 2);
                    let prefix = b / d;
                    let postfix = b % d;
                    count(prefix, steps - 1, cache) + count(postfix, steps - 1, cache)
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
