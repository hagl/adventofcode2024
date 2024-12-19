use std::{collections::HashMap, fs};

fn task1(root: &Tree, designs: &Vec<String>) -> usize {
    designs
        .iter()
        .filter(|design| is_possible(design, &root, &root))
        .count()
}

fn task2(root: &Tree, designs: &Vec<String>) -> usize {
    let mut cache: HashMap<String, usize> = HashMap::new();

    designs
        .iter()
        .map(|design| count_possible(design, &root, &root, &mut cache))
        .sum()
}

fn is_possible(s: &str, tree: &Tree, root: &Tree) -> bool {
    match s.chars().next() {
        None => true,
        Some(c) => match tree.branch(c) {
            None => false,
            Some(b) => {
                let next_tree = b.as_ref();
                let next_s: &str = &s[1..];
                is_possible(next_s, next_tree, root)
                    || (next_tree.is_leaf && is_possible(next_s, root, root))
            }
        },
    }
}

fn count_possible(s: &str, tree: &Tree, root: &Tree, cache: &mut HashMap<String, usize>) -> usize {
    if tree == root {
        if let Some(res) = cache.get(s) {
            return *res;
        }
    }
    match s.chars().next() {
        None => {
            if tree.is_leaf {
                1
            } else {
                0
            }
        }
        Some(c) => match tree.branch(c) {
            None => 0,
            Some(b) => {
                let next_tree = b.as_ref();
                let next_s: &str = &s[1..];
                let count = count_possible(next_s, next_tree, root, cache);
                if next_tree.is_leaf {
                    let c2 = count_possible(next_s, root, root, cache);
                    cache.insert(next_s.to_string(), c2);
                    count + c2
                } else {
                    count
                }
            }
        },
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Tree {
    is_leaf: bool,
    branches: [Option<Box<Tree>>; 5],
}

impl Tree {
    fn new() -> Tree {
        Tree {
            is_leaf: false,
            branches: [None, None, None, None, None],
        }
    }

    fn color_index(c: char) -> usize {
        match c {
            'w' => 0,
            'u' => 1,
            'b' => 2,
            'r' => 3,
            'g' => 4,
            c => todo!("Unexpected char: {}", c),
        }
    }

    fn print(self: &Tree, prefix: &str) {
        println!("{}", self.is_leaf);
        for c in "wubrg".chars() {
            print!("{}{} ", prefix, c);
            match self.branch(c) {
                None => println!(),
                Some(b) => b.as_ref().print(format!("  {}", prefix).as_str()),
            }
        }
    }

    fn branch(self: &Tree, c: char) -> &Option<Box<Tree>> {
        &self.branches[Tree::color_index(c)]
    }

    fn mut_branch<'a>(self: &'a mut Tree, c: char) -> &'a mut Tree {
        let o: &mut Option<Box<Tree>> = &mut self.branches[Tree::color_index(c)];
        if o.is_none() {
            let b = Box::new(Tree::new());
            *o = Some(b);
        }
        (self.branches[Tree::color_index(c)])
            .as_mut()
            .unwrap()
            .as_mut()
    }

    fn add(self: &mut Tree, iter: &mut dyn Iterator<Item = char>) {
        match iter.next() {
            None => {
                self.is_leaf = true;
            }
            Some(c) => {
                self.mut_branch(c).add(iter);
            }
        };
    }
}

pub fn solve() -> String {
    let contents = fs::read_to_string("data/day19/input.txt").unwrap();
    // let contents = fs::read_to_string("data/day19/ex.txt").unwrap();

    let mut iter = contents.split('\n');
    let patterns: Vec<String> = iter
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.to_string())
        .collect();

    let designs = iter
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect();

    // println!("{:?} {:?}", patterns, designs);
    let mut root = Tree::new();
    for p in patterns {
        root.add(&mut p.chars().into_iter())
    }
    // root.print("");

    format!(
        "Task1: {}\nTask2: {}",
        task1(&root, &designs),
        task2(&root, &designs),
    )
}
