use clap::Parser;

mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod grid;

#[derive(Parser, Debug)]
#[clap(author = "Harald Gliebe", version, about)]
/// Application configuration
struct Args {
    // run all days
    #[arg(short = 'a')]
    all: bool,
}

fn main() {
    let args = Args::parse();
    if args.all {
        println!("Day03\n{}\n", day03::solve());
        println!("Day04\n{}\n", day04::solve());
        println!("Day05\n{}\n", day05::solve());
        println!("Day06\n{}\n", day06::solve());
        println!("Day07\n{}\n", day07::solve());
        println!("Day08\n{}\n", day08::solve());
        println!("Day09\n{}\n", day09::solve());
        println!("Day10\n{}\n", day10::solve());
        println!("Day11\n{}\n", day11::solve());
        println!("Day12\n{}\n", day12::solve());
        println!("Day13\n{}\n", day13::solve());
        println!("Day14\n{}\n", day14::solve());
        println!("Day15\n{}\n", day15::solve());
        println!("Day16\n{}\n", day16::solve());
        println!("Day17\n{}\n", day17::solve());
        println!("Day18\n{}\n", day18::solve());
        println!("Day19\n{}\n", day19::solve());
        println!("Day20\n{}\n", day20::solve());
        println!("Day21\n{}\n", day21::solve());
    }
    println!("Day22\n{}\n", day22::solve());
}
