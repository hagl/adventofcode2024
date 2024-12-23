use std::{
    collections::{HashMap, HashSet},
    fs,
};

use itertools::{intersperse, Itertools};

fn task1(input: &Vec<(String, String)>) -> usize {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut connections: HashSet<(&str, &str)> = HashSet::new();

    for (s1, s2) in input {
        connections.insert((s1, s2));
        connections.insert((s2, s1));
        let entry = map.entry(s1).or_insert(vec![]);
        entry.push(s2);
        let entry = map.entry(s2).or_insert(vec![]);
        entry.push(s1);
    }

    let mut triples: HashSet<(&str, &str, &str)> = HashSet::new();

    for (s1, list) in map {
        for s2 in list.clone() {
            for s3 in list.clone() {
                if s2 != s3 && connections.contains(&(s2, s3)) {
                    let tuple = vec![s1, s2, s3]
                        .iter()
                        .sorted()
                        .cloned()
                        .collect_tuple()
                        .unwrap();
                    triples.insert(tuple);
                }
            }
        }
    }

    triples
        .iter()
        .filter(|(s1, s2, s3)| s1.starts_with("t") || s2.starts_with("t") || s3.starts_with("t"))
        .count()
}

fn task2(input: &Vec<(String, String)>) -> String {
    let mut nodes: HashSet<&str> = HashSet::new();
    let mut connections: HashSet<(&str, &str)> = HashSet::new();

    for (s1, s2) in input {
        connections.insert((s1, s2));
        connections.insert((s2, s1));
        nodes.insert(s1);
        nodes.insert(s2);
    }

    let mut p = max_clique(&mut nodes.iter().cloned().collect(), &connections);

    p.sort();

    intersperse(p, ",").collect::<String>()
}

fn max_clique<'a>(
    list: &mut Vec<&'a str>,
    connections: &HashSet<(&'a str, &'a str)>,
) -> Vec<&'a str> {
    if let Some(e) = list.pop() {
        let mut c1 = max_clique(&mut list.iter().map(|i| *i).collect(), connections);
        let mut c2 = max_clique(
            &mut list
                .iter()
                .cloned()
                .filter(|f| connections.contains(&(e, *f)))
                .collect::<Vec<&'a str>>(),
            connections,
        );
        c2.push(e);
        if c1.len() > c2.len() {
            c1
        } else {
            c2
        }
    } else {
        vec![]
    }
}

pub fn solve() -> String {
    let contents = fs::read_to_string("data/day23/input.txt").unwrap();
    // let contents = fs::read_to_string("data/day23/ex.txt").unwrap();
    let array: Vec<(String, String)> = contents
        .split("\n")
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .map(|s| s.split('-').map(|t| t.to_string()).collect_tuple().unwrap())
        .collect();

    format!("Task1: {},\nTask2: {}", task1(&array), task2(&array))
}
