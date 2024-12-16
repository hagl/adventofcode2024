use std::{fs, usize};

use nom::{
    bytes::complete::tag, character::complete::newline, multi::many1, sequence::tuple, IResult,
};

fn task1(robots: &Vec<Robot>, d: (i32, i32)) -> usize {
    move_steps(robots, d, 100).0
}

fn move_steps(robots: &Vec<Robot>, (w, h): (i32, i32), steps: i32) -> (usize, Vec<(i32, i32)>) {
    let positions: Vec<(i32, i32)> = robots
        .iter()
        .map(
            |Robot {
                 pos: (x, y),
                 vel: (vx, vy),
             }| ((x + steps * vx) % w, (y + steps * vy) % h),
        )
        .map(|(x, y)| (if x < 0 { x + w } else { x }, if y < 0 { y + h } else { y }))
        .collect();
    let mx = w / 2;
    let my: i32 = h / 2;
    let q1 = positions.iter().filter(|(x, y)| *x < mx && *y < my).count();
    let q2 = positions.iter().filter(|(x, y)| *x < mx && *y > my).count();
    let q3 = positions.iter().filter(|(x, y)| *x > mx && *y < my).count();
    let q4 = positions.iter().filter(|(x, y)| *x > mx && *y > my).count();
    (q1 * q2 * q3 * q4, positions)
}

fn task2(robots: &Vec<Robot>, (w, h): (i32, i32)) -> i32 {
    let mut min_score = usize::MAX;
    let mut min_steps = i32::MAX;
    let mut min_positions: Vec<(i32, i32)> = vec![];

    for steps in 0..(w * h) {
        let (score, positions) = move_steps(robots, (w, h), steps);
        if score < min_score {
            min_score = score;
            min_steps = steps;
            min_positions = positions;
        }
    }

    for y in 0..w {
        for x in 0..h {
            if min_positions.contains(&(x, y)) {
                print!("█");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    min_steps
}

#[derive(Debug, PartialEq)]
struct Robot {
    pos: (i32, i32),
    vel: (i32, i32),
}

fn robot(input: &str) -> IResult<&str, Robot> {
    let (input, (_, pos_x, _, pos_y, _, vel_x, _, vel_y, _)) = tuple((
        tag("p="),
        nom::character::complete::i32,
        tag(","),
        nom::character::complete::i32,
        tag(" v="),
        nom::character::complete::i32,
        tag(","),
        nom::character::complete::i32,
        newline,
    ))(input)?;
    Ok((
        input,
        Robot {
            pos: (pos_x, pos_y),
            vel: (vel_x, vel_y),
        },
    ))
}

fn robots(input: &str) -> IResult<&str, Vec<Robot>> {
    many1(robot)(input)
}

pub fn solve() -> String {
    let contents = fs::read_to_string("data/day14/input.txt").unwrap();
    // let contents = fs::read_to_string("data/day14/ex.txt").unwrap();

    let (rest, robots): (&str, Vec<Robot>) = robots(&contents).unwrap();

    // println!("{:?}", robots);
    assert_eq!(rest, "");

    format!(
        "Task1: {:?},\nTask2: {}",
        task1(&robots, (101, 103)), //73979136
        // task1(&robots, (11, 7)), //73979136
        task2(&robots, (101, 103))
    )
}
