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
pub enum Term<'a> {
    Literal {
        name: &'a str,
    },
    Expression {
        a: Box<Term<'a>>,
        b: Box<Term<'a>>,
        op: Operation,
        name: &'a str,
    },
}

impl<'a> Term<'a> {
    fn to_string(self: &Term<'a>) -> String {
        match self {
            Term::Literal { name } => name.to_string(),
            Term::Expression { a, b, op, name } => {
                format!("({}) {}<{}> ({})", a.to_string(), op, name, b.to_string())
            }
        }
    }

    fn size(self: &Term<'a>) -> usize {
        match self {
            Term::Literal { name: _ } => 1,
            Term::Expression { a, b, .. } => a.size() + b.size(),
        }
    }

    fn name_to_level(t: &str) -> usize {
        t[1..].parse().unwrap()
    }

    fn level(self: &Term<'a>) -> usize {
        match self {
            Term::Literal { name } => Term::name_to_level(*name),
            Term::Expression { a, b, .. } => std::cmp::max(a.level(), b.level()),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Gate<'a> {
    op: Operation,
    a: &'a str,
    b: &'a str,
}

fn run<'a>(values: &mut HashMap<&'a str, bool>, rules: &HashMap<&'a str, Gate>) {
    let mut remaining: Vec<&str> = rules.keys().map(|e| *e).collect();
    while !remaining.is_empty() {
        let mut next_remaining: Vec<&str> = vec![];
        for s in remaining {
            let g = rules.get(s).unwrap();
            if let Some(res) = apply(g, values) {
                values.insert(s, res);
            } else {
                next_remaining.push(s);
            }
        }
        remaining = next_remaining;
    }
}

fn task1(start_values: &Vec<(&str, bool)>, rules_vec: &Vec<(&str, Gate)>) -> i64 {
    let mut values: HashMap<&str, bool> = HashMap::new();
    for (s, v) in start_values.clone() {
        values.insert(s, v);
    }

    let mut rules: HashMap<&str, Gate> = HashMap::new();
    for (s, v) in rules_vec {
        rules.insert(s, v.clone());
    }

    let mut remaining: Vec<&str> = rules.keys().map(|e| *e).collect();
    while !remaining.is_empty() {
        let mut next_remaining: Vec<&str> = vec![];
        for s in remaining {
            let g = rules.get(s).unwrap();
            if let Some(res) = apply(g, &mut values) {
                values.insert(s, res);
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

fn printBinary(values: &HashMap<&str, bool>, prefix: &str) {
    let res: String = values
        .keys()
        .filter(|s| s.starts_with(prefix))
        .sorted()
        .rev()
        .map(|v| if *values.get(v).unwrap() { "1" } else { "0" })
        .collect();
    println!("{}", res);
}

// fn swap_rules<'a>(n1: &'a str, n2: &'a str, rules: &'a mut HashMap<&'a str, Gate>) {
//     let rule_1 = rules.get(n1).unwrap().clone();
//     let rule_2 = rules.get(n2).unwrap().clone();
//     rules.insert(n1, rule_2);
//     rules.insert(n2, rule_1);
// }

fn task2(_start_values: Vec<(&str, bool)>, rules_vec: &Vec<(&str, Gate)>) -> i64 {
    let mut xs: Vec<String> = vec![];
    let mut ys: Vec<String> = vec![];

    let mut rules: HashMap<&str, Gate> = HashMap::new();
    for (s, v) in rules_vec {
        rules.insert(s, v.clone());
    }

    // cqm,mps,vcv,vjv,vwp,z13,z19,z25
    // swap vcv , z13
    let rule_vcv = rules.get("vcv").unwrap().clone();
    let rule_z13 = rules.get("z13").unwrap().clone();
    rules.insert("vcv", rule_z13);
    rules.insert("z13", rule_vcv);
    // swap vwp, z19
    let rule_vwp = rules.get("vwp").unwrap().clone();
    let rule_z19 = rules.get("z19").unwrap().clone();
    rules.insert("vwp", rule_z19);
    rules.insert("z19", rule_vwp);
    // swap mps, z25
    let rule_mps = rules.get("mps").unwrap().clone();
    let rule_z25 = rules.get("z25").unwrap().clone();
    rules.insert("mps", rule_z25);
    rules.insert("z25", rule_mps);

    // // swap qnw, z33
    // swap cqm, vjv
    let rule_cqm = rules.get("cqm").unwrap().clone();
    let rule_vjv = rules.get("vjv").unwrap().clone();
    rules.insert("cqm", rule_vjv);
    rules.insert("vjv", rule_cqm);

    for i in 0..=45 {
        xs.push(format!("x{:02}", i));
        ys.push(format!("y{:02}", i));
    }
    for i in 0..=44 {
        let mut values: HashMap<&str, bool> = HashMap::new();
        for j in 0..=45 {
            values.insert(&xs[j], j == i);
            // values.insert(&xs[j], false);
            values.insert(&ys[j], j == i);
            // values.insert(&ys[j], false);
        }
        run(&mut values, &rules);
        println!("---- {}", i);
        printBinary(&values, "x");
        printBinary(&values, "y");
        printBinary(&values, "z");
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

fn resolve<'a>(name: &'a str, rules: &'a HashMap<&'a str, Gate>, top_level: bool) -> Term<'a> {
    if name.starts_with("x") || name.starts_with("y") || !top_level && name.starts_with("z") {
        Term::Literal { name }
    } else {
        if let Some(Gate { a, b, op }) = rules.get(name) {
            let at = resolve(a, rules, false);
            let bt = resolve(b, rules, false);
            if at.level() >= bt.level() {
                Term::Expression {
                    a: Box::new(at),
                    b: Box::new(bt),
                    op: op.clone(),
                    name,
                }
            } else {
                Term::Expression {
                    a: Box::new(bt),
                    b: Box::new(at),
                    op: op.clone(),
                    name,
                }
            }
        } else {
            Term::Literal { name }
        }
    }
}

fn apply(g: &Gate, values: &mut HashMap<&str, bool>) -> Option<bool> {
    let op1 = values.get(g.a)?;
    let op2 = values.get(g.b)?;
    match g.op {
        Operation::AND => Some(*op1 && *op2),
        Operation::OR => Some(*op1 || *op2),
        Operation::XOR => Some(*op1 ^ *op2),
    }
}

/*

x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02

*/

fn startValue(input: &str) -> IResult<&str, (&str, bool)> {
    let (input, (name, _, value, _)) = tuple((
        nom::character::complete::alphanumeric1,
        tag(": "),
        nom::character::complete::i64,
        newline,
    ))(input)?;
    Ok((input, (name, value == 1)))
}

fn operation(input: &str) -> IResult<&str, Operation> {
    alt((
        nom::combinator::map(tag("AND"), |_| Operation::AND),
        nom::combinator::map(tag("OR"), |_| Operation::OR),
        nom::combinator::map(tag("XOR"), |_| Operation::XOR),
    ))(input)
}

fn rule(input: &str) -> IResult<&str, (&str, Gate)> {
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
    Ok((input, (name, Gate { a, b, op })))
}

fn configurations(input: &str) -> IResult<&str, (Vec<(&str, bool)>, Vec<(&str, Gate)>)> {
    let (input, (start, _, rules)) = tuple((many1(startValue), newline, many1(rule)))(input)?;
    Ok((input, (start, rules)))
}

pub fn solve() -> String {
    let contents = fs::read_to_string("data/day24/input.txt").unwrap();
    // let contents = fs::read_to_string("data/day24/ex.txt").unwrap();

    let (rest, (start_values, rules)): (&str, (Vec<(&str, bool)>, Vec<(&str, Gate)>)) =
        configurations(&contents).unwrap();

    // println!("{:?}\n\n{:?}", start_values, rules);
    assert_eq!(rest, "");

    format!(
        "Task1: {:?},\nTask2: {}",
        task1(&start_values, &rules),
        task2(start_values, &rules),
    )
}
