use itertools::intersperse;
use std::fs;

use nom::{
    bytes::complete::tag,
    character::complete::newline,
    multi::{many0, separated_list1},
    sequence::tuple,
    IResult,
};

fn combo(state: &State, arg: i64) -> i64 {
    match arg {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => state.a,
        5 => state.b,
        6 => state.c,
        c => todo!("{}", c),
    }
}

fn task1(state: &State, instructions: &Vec<i64>) -> String {
    let output = run(&mut state.clone(), instructions);
    intersperse(
        output
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>(),
        ",".to_string(),
    )
    .collect::<String>()
}

fn run(state: &mut State, instructions: &Vec<i64>) -> Vec<i64> {
    let len = instructions.len();
    let mut result: Vec<i64> = vec![];
    while state.i < len {
        let instruction = instructions[state.i];
        let arg = instructions[state.i + 1];
        match instruction {
            0 => {
                // adv
                state.a = state.a >> combo(&state, arg);
                state.i = state.i + 2;
            }
            1 => {
                // bxl
                state.b = state.b ^ arg;
                state.i = state.i + 2;
            }
            2 => {
                // bst
                state.b = combo(state, arg) % 8;
                state.i = state.i + 2;
            }
            3 => {
                // jnz
                state.i = if state.a == 0 {
                    state.i + 2
                } else {
                    arg.try_into().unwrap()
                };
            }
            4 => {
                // bxc
                state.b = state.b ^ state.c;
                state.i = state.i + 2;
            }
            5 => {
                // out
                result.push(combo(&state, arg) % 8);
                state.i = state.i + 2;
            }
            6 => {
                // bdv
                state.b = state.a >> combo(&state, arg);
                state.i = state.i + 2;
            }
            7 => {
                // cdv
                state.c = state.a >> combo(&state, arg);
                state.i = state.i + 2;
            }
            _ => todo!(),
        }
    }
    result
}

#[derive(Debug, PartialEq, Clone)]
struct State {
    i: usize,
    a: i64,
    b: i64,
    c: i64,
}

// Register A: 729

pub fn register(name: char) -> impl Fn(&str) -> IResult<&str, i64> {
    move |input| {
        let t = format!("Register {}: ", name);
        let (input, (_, value, _)) =
            tuple((tag(&t[0..]), nom::character::complete::i64, newline))(input)?;
        Ok((input, value))
    }
}
fn state(input: &str) -> IResult<&str, State> {
    let (input, (a, b, c, _)) =
        tuple((register('A'), register('B'), register('C'), many0(newline)))(input)?;

    Ok((input, State { i: 0, a, b, c }))
}

fn instructions(input: &str) -> IResult<&str, Vec<i64>> {
    let (input, (_, instructions)) = tuple((
        tag("Program: "),
        separated_list1(tag(","), nom::character::complete::i64),
    ))(input)?;
    Ok((input, instructions))
}

fn configuration(input: &str) -> IResult<&str, (State, Vec<i64>)> {
    let (input, (state, instructions, _)) = tuple((state, instructions, many0(newline)))(input)?;

    Ok((input, (state, instructions)))
}

pub fn solve() -> String {
    let contents = fs::read_to_string("data/day17/input.txt").unwrap();
    // let contents = fs::read_to_string("data/day17/ex.txt").unwrap();

    let (rest, (mut state, instructions)): (&str, (State, Vec<i64>)) =
        configuration(&contents).unwrap();

    assert_eq!(rest, "");

    format!(
        "Task1: {}\nTask2: {}",
        task1(&mut state, &instructions),
        task2(&mut state, &instructions),
    )
}

fn task2(state: &State, instructions: &Vec<i64>) -> i64 {
    let mut solutions = go(state, instructions, 0, instructions);
    solutions.sort();
    solutions[0]
}

fn go(state: &State, instructions: &Vec<i64>, num: i64, expected: &Vec<i64>) -> Vec<i64> {
    if expected.is_empty() {
        vec![num]
    } else {
        let mut expected = expected.clone();
        let r = expected.pop().unwrap();
        let candidates = (0..8).filter(|ix| {
            let mut s = state.clone();
            s.a = (num << 3) + ix;
            let output = run(&mut s, instructions);
            if let Some(n) = output.iter().next() {
                *n == r
            } else {
                false
            }
        });
        let solutions: Vec<i64> = candidates
            .flat_map(|c| go(state, instructions, (num << 3) + c, &expected))
            .collect();
        solutions
    }
}

// Program: 2,4,1,3,7,5,1,5,0,3,4,3,5,5,3,0
// 2,4 : bst 4 : A % 8 -> B
// 1,3 : bxl 3 : B ^ 3 -> B
// 7,5 : cdv 5 : A >> B -> C
// 1,5 : bxl 5 : B ^ 5 -> B
// 0,3 : adv 3 : A >> 3 -> A
// 4,3 : bxc : B ^ C -> B
// 5,5 : out : print B % 8
// 3,0 : jnz ; if A != 0 goto 0
