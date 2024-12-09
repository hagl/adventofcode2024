use clap::Parser;

mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

#[derive(Parser, Debug)]
#[clap(author = "Harald Gliebe", version, about)]
/// Application configuration
struct Args {
    /// whether to be verbose
    #[arg(short = 'v')]
    verbose: bool,

    /// an optional name to greet
    #[arg()]
    name: Option<String>,
}

fn main() {
    let args = Args::parse();
    if args.verbose {
        println!("DEBUG {args:?}");
    }
    println!(
        "Hello {} (from adventofcode2024)!",
        args.name.unwrap_or("world".to_string())
    );

    println!("Day03\n{}\n", day03::solve());
    println!("Day04\n{}\n", day04::solve());
    println!("Day05\n{}\n", day05::solve());
    println!("Day06\n{}\n", day06::solve());
    println!("Day07\n{}\n", day07::solve());
    println!("Day08\n{}\n", day08::solve());
    println!("Day09\n{}\n", day09::solve());
}
