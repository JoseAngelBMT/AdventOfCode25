use clap::Parser;
use anyhow::Result;

mod days;
mod board;

#[derive(Parser)]
struct Cli {
    #[arg(short, long, value_name = "DAY")]
    day: u8,

    #[arg(short, long, value_name = "PART")]
    part: Option<u8>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    println!("Advent of Code - DAY {} ", cli.day);

    match cli.day {
        1 => days::day01::solve()?,
        2 => days::day02::solve()?,
        3 => days::day03::solve()?,
        4 => days::day04::solve()?,
        5 => days::day05::solve()?,
        6 => days::day06::solve()?,
        7 => days::day07::solve()?,
        8 => days::day08::solve()?,
        9 => days::day09::solve()?,
        10 => days::day10::solve()?,
        11 => days::day11::solve()?,
        12 => days::day12::solve()?,
        _ => println!("Day {} not implemented.", cli.day),
    }

    Ok(())
}
