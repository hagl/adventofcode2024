use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn mix(s: i64, v: i64) -> i64 {
    s ^ v
}

fn prune(s: i64) -> i64 {
    s % 16777216
}

fn task1(input: &Vec<i64>) -> i64 {
    input.iter().map(|s| iterate(*s, 2000)).sum()
}

fn task2(input: &Vec<i64>) -> i64 {
    let mut global_cache: HashMap<(i8, i8, i8, i8), i64> = HashMap::new();
    input.iter().for_each(|s| {
        let mut local_cache: HashSet<(i8, i8, i8, i8)> = HashSet::new();
        let prices: Vec<i8> = (0..2000)
            .scan(*s, |acc, ix| {
                *acc = next_random(*acc);
                Some(*acc)
            })
            .map(|s| (s % 10).try_into().unwrap())
            .collect();
        prices[..].windows(5).for_each(|a| {
            let key = (a[1] - a[0], a[2] - a[1], a[3] - a[2], a[4] - a[3]);
            if local_cache.insert(key) {
                let entry = global_cache.entry(key).or_insert(0);
                let v: i64 = a[4].try_into().unwrap();
                *entry += v
            };
        });
    });
    *global_cache.values().max().unwrap()
}

fn iterate(s: i64, n: usize) -> i64 {
    let mut s = s;
    for _ in 0..n {
        s = next_random(s);
    }
    s
}

fn next_random(s: i64) -> i64 {
    let s = prune(mix(s, s << 6));
    let s = prune(mix(s, s >> 5));
    let s = prune(mix(s, s << 11));
    s
}

pub fn solve() -> String {
    let contents = fs::read_to_string("data/day22/input.txt").unwrap();
    // fs::read_to_string("data/day22/ex.txt").unwrap();
    let array: Vec<i64> = contents
        .split("\n")
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();

    format!("Task1: {},\nTask2: {}", task1(&array), task2(&array))
}
