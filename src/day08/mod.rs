use num::integer::gcd;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn task2(antenas: &HashMap<char, Vec<(i32, i32)>>, width: usize, height: usize) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let w32: i32 = width.try_into().unwrap();
    let h32: i32 = height.try_into().unwrap();
    for list in antenas.values() {
        for i in 0..list.len() {
            for j in (i + 1)..list.len() {
                let (x1, y1) = list[i];
                let (x2, y2) = list[j];
                let deltax = x2 - x1;
                let deltay = y2 - y1;
                let d = gcd(deltax, deltay);
                let dx = deltax / d;
                let dy = deltay / d;
                for i in -w32..(w32 + 1) {
                    let x = x1 + i * dx;
                    let y = y1 + i * dy;
                    if x >= 0 && y >= 0 && x < w32 && y < h32 {
                        visited.insert((x, y));
                    }
                }
            }
        }
    }
    visited.len()
}

fn task1(antenas: &HashMap<char, Vec<(i32, i32)>>, width: usize, height: usize) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let w32: i32 = width.try_into().unwrap();
    let h32: i32 = height.try_into().unwrap();
    for list in antenas.values() {
        for i in 0..list.len() {
            for j in (i + 1)..list.len() {
                let (x1, y1) = list[i];
                let (x2, y2) = list[j];
                let a1x = x1 - (x2 - x1);
                let a1y = y1 - (y2 - y1);
                let a2x = x2 + (x2 - x1);
                let a2y = y2 + (y2 - y1);
                if a1x >= 0 && a1y >= 0 && a1x < w32 && a1y < h32 {
                    visited.insert((a1x, a1y));
                }
                if a2x >= 0 && a2y >= 0 && a2x < w32 && a2y < h32 {
                    visited.insert((a2x, a2y));
                }
            }
        }
    }
    visited.len()
}

pub fn solve() -> String {
    let contents =
        fs::read_to_string("data/day08/input.txt").expect("Should have been able to read the file");
    // fs::read_to_string("data/day08/ex.txt").expect("Should have been able to read the file");
    let mut antenas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let array: Vec<Vec<char>> = contents
        .split("\n")
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect())
        .collect();
    let height = array.len();
    let width = array[0].len();
    // find start position

    for x in 0..width {
        for y in 0..height {
            let line = &array[y];
            let c = line[x];
            if c != '.' {
                antenas
                    .entry(c)
                    .or_insert(vec![])
                    .push((x.try_into().unwrap(), y.try_into().unwrap())); //<(usize, usize)>)
            }
        }
    }

    format!(
        "Task1: {},\nTask2: {}",
        task1(&antenas, width, height),
        task2(&antenas, width, height)
    )
}
