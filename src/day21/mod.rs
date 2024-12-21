use std::{collections::HashMap, fs};

/*
| 7 | 8 | 9 |
| 4 | 5 | 6 |
| 1 | 2 | 3 |
     | 0 | A |
*/

fn number_coord(n: char) -> (usize, usize) {
    match n {
        '0' => (1, 0),
        'A' => (2, 0),
        '1' => (0, 1),
        '2' => (1, 1),
        '3' => (2, 1),
        '4' => (0, 2),
        '5' => (1, 2),
        '6' => (2, 2),
        '7' => (0, 3),
        '8' => (1, 3),
        '9' => (2, 3),
        c => todo!("Unexpected number: {}", c),
    }
}

fn dir_coord(n: char) -> (usize, usize) {
    match n {
        '<' => (0, 0),
        'v' => (1, 0),
        '>' => (2, 0),
        '^' => (1, 1),
        'A' => (2, 1),
        c => todo!("Unexpected number: {}", c),
    }
}

fn dir_paths(f: char, t: char) -> Vec<Vec<char>> {
    coord_dir_paths(dir_coord(f), dir_coord(t))
}

/*
    | ^ | A |
| < | v | > |
*/

fn coord_dir_paths((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> Vec<Vec<char>> {
    let mut x_first = true;
    let mut y_first = true;
    let dx = if x2 > x1 {
        vec!['>'; x2 - x1]
    } else if x2 == x1 {
        x_first = false;
        vec![]
    } else {
        if y1 == 1 && x2 == 0 {
            x_first = false;
        }
        vec!['<'; x1 - x2]
    };
    let dy = if y2 > y1 {
        if x1 == 0 && y2 == 1 {
            y_first = false;
        }
        vec!['^'; y2 - y1]
    } else if y2 == y1 {
        y_first = false;
        vec![]
    } else {
        vec!['v'; y1 - y2]
    };
    let mut result: Vec<Vec<char>> = vec![];
    if x_first {
        let mut e = dx.clone();
        e.append(&mut dy.clone());
        e.push('A');
        result.push(e);
    }
    if y_first {
        let mut e = dy.clone();
        e.append(&mut dx.clone());
        e.push('A');
        result.push(e);
    }
    if !x_first && !y_first {
        result.push(vec!['A']);
    }
    result
}

fn number_paths(f: char, t: char) -> Vec<Vec<char>> {
    coord_number_paths(number_coord(f), number_coord(t))
}

fn coord_number_paths((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> Vec<Vec<char>> {
    let mut x_first = true;
    let mut y_first = true;
    let dx = if x2 > x1 {
        vec!['>'; x2 - x1]
    } else if x2 == x1 {
        x_first = false;
        vec![]
    } else {
        if y1 == 0 && x2 == 0 {
            x_first = false;
        }
        vec!['<'; x1 - x2]
    };
    let dy = if y2 > y1 {
        vec!['^'; y2 - y1]
    } else if y2 == y1 {
        y_first = false;
        vec![]
    } else {
        if x1 == 0 && y2 == 0 {
            y_first = false;
        }
        vec!['v'; y1 - y2]
    };
    let mut result: Vec<Vec<char>> = vec![];
    if x_first {
        let mut e = dx.clone();
        e.append(&mut dy.clone());
        e.push('A');
        result.push(e);
    }
    if y_first {
        let mut e = dy.clone();
        e.append(&mut dx.clone());
        e.push('A');
        result.push(e);
    }
    if !x_first && !y_first {
        result.push(vec!['A']);
    }
    result
}

fn task1(input: &Vec<String>) -> usize {
    let mut n_paths: HashMap<(char, char), Vec<Vec<char>>> = HashMap::new();
    let mut d_paths: HashMap<(char, char), Vec<Vec<char>>> = HashMap::new();

    let mut numbers: Vec<char> = ('0'..='9').collect();
    numbers.push('A');

    for i in numbers.clone() {
        for j in numbers.clone() {
            n_paths.insert((i, j), number_paths(i, j));
        }
    }

    let dirs: Vec<char> = vec!['^', '>', 'v', '<', 'A'];
    for i in dirs.clone() {
        for j in dirs.clone() {
            d_paths.insert((i, j), dir_paths(i, j));
        }
    }

    input
        .iter()
        .map(|s| {
            let paths = paths0(
                'A',
                &mut s.chars().rev().collect(),
                vec![],
                &n_paths,
                &d_paths,
            );
            // println!(
            //     "{:?}",
            //     paths
            //         .iter()
            //         .map(|cs| cs.iter().cloned().collect::<String>())
            //         .collect::<Vec<String>>()
            // );
            let len = paths.iter().map(|cs| cs.len()).min().unwrap();
            let num: usize = s
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse()
                .unwrap();
            println!("{}: {} {}", s, len, num);
            len * num
        })
        .sum()
}

fn paths0(
    last: char,
    chars: &mut Vec<char>,
    prefix: Vec<char>,
    n_paths: &HashMap<(char, char), Vec<Vec<char>>>,
    d_paths: &HashMap<(char, char), Vec<Vec<char>>>,
) -> Vec<Vec<char>> {
    if let Some(c) = chars.pop() {
        // println!("{} {}", last, c);
        let ps = n_paths.get(&(last, c)).unwrap();
        ps.iter()
            .flat_map(|p| {
                let mut new_prefix = prefix.clone();
                new_prefix.append(&mut p.clone());
                paths0(c, &mut chars.clone(), new_prefix, n_paths, d_paths)
            })
            .collect()
    } else {
        // vec![prefix]

        paths1(
            'A',
            &mut prefix.iter().cloned().rev().collect(),
            vec![],
            d_paths,
            1,
        )
    }
}

fn paths1(
    last: char,
    chars: &mut Vec<char>,
    prefix: Vec<char>,
    d_paths: &HashMap<(char, char), Vec<Vec<char>>>,
    level: usize,
) -> Vec<Vec<char>> {
    if let Some(c) = chars.pop() {
        // println!("{} {}", last, c);
        let ps = d_paths.get(&(last, c)).unwrap();
        ps.iter()
            .flat_map(|p| {
                let mut new_prefix = prefix.clone();
                new_prefix.append(&mut p.clone());
                paths1(c, &mut chars.clone(), new_prefix, d_paths, level)
            })
            .collect()
    } else {
        if level == 0 {
            vec![prefix]
        } else {
            paths1(
                'A',
                &mut prefix.iter().cloned().rev().collect(),
                vec![],
                d_paths,
                level - 1,
            )
        }
    }
}

fn paths2(
    last: char,
    chars: &mut Vec<char>,
    prefix: Vec<char>,
    n_paths: &HashMap<(char, char), Vec<Vec<char>>>,
) -> Vec<Vec<char>> {
    if let Some(c) = chars.pop() {
        // println!("{} {}", last, c);
        let ps = n_paths.get(&(last, c)).unwrap();
        ps.iter()
            .flat_map(|p| {
                let mut new_prefix = prefix.clone();
                new_prefix.append(&mut p.clone());
                paths2(c, &mut chars.clone(), new_prefix, n_paths)
            })
            .collect()
    } else {
        vec![prefix]
    }
}

fn task2(input: &Vec<String>) -> usize {
    let mut n_paths: HashMap<(char, char), Vec<Vec<char>>> = HashMap::new();
    let mut d_paths: HashMap<(char, char), Vec<Vec<char>>> = HashMap::new();

    let mut numbers: Vec<char> = ('0'..='9').collect();
    numbers.push('A');

    for i in numbers.clone() {
        for j in numbers.clone() {
            n_paths.insert((i, j), number_paths(i, j));
        }
    }

    let dirs: Vec<char> = vec!['^', '>', 'v', '<', 'A'];
    for i in dirs.clone() {
        for j in dirs.clone() {
            d_paths.insert((i, j), dir_paths(i, j));
        }
    }

    let mut d_costs: HashMap<(char, char), usize> = HashMap::new();
    for i in dirs.clone() {
        for j in dirs.clone() {
            d_costs.insert((i, j), 1);
        }
    }

    for _ in 0..25 {
        let previous_costs = d_costs.clone();
        for i in dirs.clone() {
            for j in dirs.clone() {
                let cost = d_paths
                    .get(&(i, j))
                    .unwrap()
                    .iter()
                    .map(|path| {
                        path.iter()
                            .fold(('A', 0), |(p, acc), c| {
                                (*c, acc + previous_costs.get(&(p, *c)).unwrap())
                            })
                            .1
                    })
                    .min()
                    .unwrap();
                d_costs.insert((i, j), cost);
            }
        }
    }

    input
        .iter()
        .map(|s| {
            let paths = paths2('A', &mut s.chars().rev().collect(), vec![], &n_paths);
            let len = paths
                .iter()
                .map(|path| {
                    path.iter()
                        .fold(('A', 0), |(p, acc), c| {
                            (*c, acc + d_costs.get(&(p, *c)).unwrap())
                        })
                        .1
                })
                .min()
                .unwrap();

            let num: usize = s
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse()
                .unwrap();
            println!("{}: {} {}", s, len, num);
            len * num
        })
        .sum()
}

pub fn solve() -> String {
    let contents = fs::read_to_string("data/day21/input.txt").unwrap();
    // fs::read_to_string("data/day21/ex.txt").unwrap();
    let array: Vec<String> = contents
        .split("\n")
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect();
    format!("Task1: {},\nTask2: {}", task1(&array), task2(&array))
}
