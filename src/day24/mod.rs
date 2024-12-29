use std::{collections::HashMap, fmt::Display, fs};

use itertools::Itertools;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::newline, multi::many1, sequence::tuple,
    IResult,
};

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Operation {
    AND,
    OR,
    XOR,
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Operation::AND => "&&",
                Operation::OR => "||",
                Operation::XOR => "^",
            }
        )
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Term {
    Literal {
        name: String,
    },
    Expression {
        a: Box<Term>,
        b: Box<Term>,
        op: Operation,
        name: String,
    },
}

impl Term {
    fn to_string(self: &Term) -> String {
        match self {
            Term::Literal { name } => name.to_string(),
            Term::Expression { a, b, op, name } => {
                format!("({}) {}<{}> ({})", a.to_string(), op, name, b.to_string())
            }
        }
    }

    fn size(self: &Term) -> usize {
        match self {
            Term::Literal { name: _ } => 1,
            Term::Expression { a, b, .. } => a.size() + b.size(),
        }
    }

    fn name_to_level(t: &str) -> usize {
        t[1..].parse().unwrap()
    }

    fn level(self: &Term) -> usize {
        match self {
            Term::Literal { name } => Term::name_to_level(name),
            Term::Expression { a, b, .. } => std::cmp::max(a.level(), b.level()),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Gate {
    op: Operation,
    a: String,
    b: String,
}

fn run(values: &mut HashMap<String, bool>, rules: &HashMap<String, Gate>) {
    let mut remaining: Vec<&str> = rules.keys().map(|s| s.as_str()).collect();
    while !remaining.is_empty() {
        let mut next_remaining: Vec<&str> = vec![];
        for s in remaining {
            let g = rules.get(s).unwrap();
            if let Some(res) = apply(g, values) {
                values.insert(s.to_string(), res);
            } else {
                next_remaining.push(&s);
            }
        }
        remaining = next_remaining;
    }
}

fn task1(start_values: &Vec<(String, bool)>, rules_vec: &Vec<(String, Gate)>) -> i64 {
    let mut values: HashMap<String, bool> = HashMap::new();
    for (s, v) in start_values.clone() {
        values.insert(s, v);
    }

    let mut rules: HashMap<String, Gate> = HashMap::new();
    for (s, v) in rules_vec {
        rules.insert(s.to_string(), v.clone());
    }

    let mut remaining: Vec<&str> = rules.keys().map(|e| e.as_str()).collect();
    while !remaining.is_empty() {
        let mut next_remaining: Vec<&str> = vec![];
        for s in remaining {
            let g = rules.get(s).unwrap();
            if let Some(res) = apply(g, &mut values) {
                values.insert(s.to_string(), res);
            } else {
                next_remaining.push(s);
            }
        }
        remaining = next_remaining;
    }

    values
        .keys()
        .filter(|s| s.starts_with("z"))
        .sorted()
        .rev()
        .fold(0, |acc, v| {
            (acc << 1) + (if *values.get(v).unwrap() { 1 } else { 0 })
        })
}

fn print_binary(values: &HashMap<String, bool>, prefix: &str) {
    let res: String = values
        .keys()
        .filter(|s| s.starts_with(prefix))
        .sorted()
        .rev()
        .map(|v| if *values.get(v).unwrap() { "1" } else { "0" })
        .collect();
    println!("{}", res);
}

fn swap_rules(n1: &str, n2: &str, rules: &mut HashMap<String, Gate>) {
    let rule_1 = rules.get(n1).unwrap().clone();
    let rule_2 = rules.get(n2).unwrap().clone();
    rules.insert(n1.to_string(), rule_2);
    rules.insert(n2.to_string(), rule_1);
}

fn task2(_start_values: Vec<(String, bool)>, rules_vec: &Vec<(String, Gate)>) -> i64 {
    let mut xs: Vec<String> = vec![];
    let mut ys: Vec<String> = vec![];

    let mut rules: HashMap<String, Gate> = HashMap::new();
    for (s, v) in rules_vec {
        rules.insert(s.clone(), v.clone());
    }

    // cqm,mps,vcv,vjv,vwp,z13,z19,z25
    // swap vcv , z13
    swap_rules("vcv", "z13", &mut rules);
    // swap vwp, z19
    swap_rules("vwp", "z19", &mut rules);
    // swap mps, z25
    swap_rules("mps", "z25", &mut rules);
    // swap cqm, vjv
    swap_rules("cqm", "vjv", &mut rules);

    for i in 0..=45 {
        xs.push(format!("x{:02}", i));
        ys.push(format!("y{:02}", i));
    }
    for i in 0..=44 {
        let mut values: HashMap<String, bool> = HashMap::new();
        for j in 0..=45 {
            values.insert(xs[j].clone(), j == i);
            // values.insert(xs[j].clone(), false);
            values.insert(ys[j].clone(), j == i);
            // values.insert(ys[j].clone(), false);
        }
        run(&mut values, &rules);
        println!("---- {}", i);
        print_binary(&values, "x");
        print_binary(&values, "y");
        print_binary(&values, "z");
    }

    for i in 0..=45 {
        let name = format!("z{:02}", i);
        let expr = resolve(&name, &rules, true);
        println!("{} ({})= {}", name, expr.size(), expr.to_string());
    }

    if let Term::Expression { a, b, .. } = resolve("z19", &rules, true) {
        println!("Levels: {} {}", a.level(), b.level());
    };

    0
}

fn resolve(name: &str, rules: &HashMap<String, Gate>, top_level: bool) -> Term {
    if name.starts_with("x") || name.starts_with("y") || !top_level && name.starts_with("z") {
        Term::Literal {
            name: name.to_string(),
        }
    } else {
        if let Some(Gate { a, b, op }) = rules.get(name) {
            let at = resolve(a, rules, false);
            let bt = resolve(b, rules, false);
            if at.level() >= bt.level() {
                Term::Expression {
                    a: Box::new(at),
                    b: Box::new(bt),
                    op: op.clone(),
                    name: name.to_string(),
                }
            } else {
                Term::Expression {
                    a: Box::new(bt),
                    b: Box::new(at),
                    op: op.clone(),
                    name: name.to_string(),
                }
            }
        } else {
            Term::Literal {
                name: name.to_string(),
            }
        }
    }
}

fn apply(g: &Gate, values: &mut HashMap<String, bool>) -> Option<bool> {
    let op1 = values.get(&g.a)?;
    let op2 = values.get(&g.b)?;
    match g.op {
        Operation::AND => Some(*op1 && *op2),
        Operation::OR => Some(*op1 || *op2),
        Operation::XOR => Some(*op1 ^ *op2),
    }
}

fn start_value(input: &str) -> IResult<&str, (String, bool)> {
    let (input, (name, _, value, _)) = tuple((
        nom::character::complete::alphanumeric1,
        tag(": "),
        nom::character::complete::i64,
        newline,
    ))(input)?;
    Ok((input, (name.to_string(), value == 1)))
}

fn operation(input: &str) -> IResult<&str, Operation> {
    alt((
        nom::combinator::map(tag("AND"), |_| Operation::AND),
        nom::combinator::map(tag("OR"), |_| Operation::OR),
        nom::combinator::map(tag("XOR"), |_| Operation::XOR),
    ))(input)
}

fn rule(input: &str) -> IResult<&str, (String, Gate)> {
    let (input, (a, _, op, _, b, _, name, _)) = tuple((
        nom::character::complete::alphanumeric1,
        tag(" "),
        operation,
        tag(" "),
        nom::character::complete::alphanumeric1,
        tag(" -> "),
        nom::character::complete::alphanumeric1,
        newline,
    ))(input)?;
    Ok((
        input,
        (
            name.to_string(),
            Gate {
                a: a.to_string(),
                b: b.to_string(),
                op,
            },
        ),
    ))
}

fn configurations(input: &str) -> IResult<&str, (Vec<(String, bool)>, Vec<(String, Gate)>)> {
    let (input, (start, _, rules)) = tuple((many1(start_value), newline, many1(rule)))(input)?;
    Ok((input, (start, rules)))
}

pub fn solve() -> String {
    let contents = fs::read_to_string("data/day24/input.txt").unwrap();
    // let contents = fs::read_to_string("data/day24/ex.txt").unwrap();

    let (rest, (start_values, rules)): (&str, (Vec<(String, bool)>, Vec<(String, Gate)>)) =
        configurations(&contents).unwrap();

    // println!("{:?}\n\n{:?}", start_values, rules);
    assert_eq!(rest, "");

    format!(
        "Task1: {:?},\nTask2: {}",
        task1(&start_values, &rules),
        task2(start_values, &rules),
    )
}
