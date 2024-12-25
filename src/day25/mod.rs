use std::fs;

fn task1(input: &Vec<u64>) -> usize {
    input
        .iter()
        .map(|key| {
            if *key & 0b11111 == 0b11111 {
                input.iter().filter(|lock| *key & *lock == 0).count()
            } else {
                0
            }
        })
        .sum()
}

pub fn solve() -> String {
    let contents = fs::read_to_string("data/day25/input.txt").unwrap();
    // let contents = fs::read_to_string("data/day25/ex.txt").unwrap();
    let array: Vec<u64> = contents
        .split("\n\n")
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.chars().fold(0, |acc, c| match c {
                '#' => (acc << 1) + 1,
                '.' => acc << 1,
                _ => acc,
            })
        })
        .collect();

    format!("Task1: {}", task1(&array))
}
