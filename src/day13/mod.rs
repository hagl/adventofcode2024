use std::fs;

use nom::{
    bytes::complete::tag,
    character::complete::newline,
    multi::{many0, many1},
    sequence::tuple,
    IResult, Parser,
};

fn tokens(configuration: &Configuration) -> i64 {
    let Configuration {
        a: (ax, ay),
        b: (bx, by),
        prize: (px, py),
    } = configuration;

    let d = ax * by - ay * bx;
    let m1 = px * by - py * bx;
    let m2 = py * ax - px * ay;
    if d == 0 {
        // This is the interesting case, but it didn't occur in the example or input
        // so left as an exerecise to the reader ;-)
        println!("{:?} \n d= {}", configuration, d);
        0
    } else {
        if m1 % d == 0 && m2 % d == 0 {
            let a = m1 / d;
            let b = m2 / d;
            if a >= 0 && b >= 0 {
                a * 3 + b
            } else {
                0
            }
        } else {
            0
        }
    }
}

fn task1(configurations: &Vec<Configuration>) -> i64 {
    configurations.iter().map(|c| tokens(c)).sum()
}
fn task2(configurations: &Vec<Configuration>) -> i64 {
    configurations
        .iter()
        .map(
            |Configuration {
                 a,
                 b,
                 prize: (px, py),
             }| Configuration {
                a: *a,
                b: *b,
                prize: (10000000000000 + px, 10000000000000 + py),
            },
        )
        .map(|c| tokens(&c))
        .sum()
}

#[derive(Debug, PartialEq)]
struct Configuration {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

fn button(input: &str) -> IResult<&str, (i64, i64)> {
    let (input, (_, x, _, y, _)) = tuple((
        tag("Button A: X+").or(tag("Button B: X+")),
        nom::character::complete::i64,
        tag(", Y+"),
        nom::character::complete::i64,
        newline,
    ))(input)?;
    Ok((input, (x, y)))
}

fn prize(input: &str) -> IResult<&str, (i64, i64)> {
    let (input, (_, x, _, y, _)) = tuple((
        tag("Prize: X="),
        nom::character::complete::i64,
        tag(", Y="),
        nom::character::complete::i64,
        newline,
    ))(input)?;
    Ok((input, (x, y)))
}

fn configuration(input: &str) -> IResult<&str, Configuration> {
    let (input, (a, b, prize, _)) = tuple((button, button, prize, many0(newline)))(input)?;

    Ok((input, Configuration { a, b, prize }))
}

fn configurations(input: &str) -> IResult<&str, Vec<Configuration>> {
    many1(configuration)(input)
}

pub fn solve() -> String {
    let contents = fs::read_to_string("data/day13/input.txt").unwrap();
    // let contents = fs::read_to_string("data/day13/ex.txt").unwrap();

    let (rest, configuration): (&str, Vec<Configuration>) = configurations(&contents).unwrap();

    assert_eq!(rest, "");

    format!(
        "Task1: {:?},\nTask2: {}",
        task1(&configuration),
        task2(&configuration)
    )
}
