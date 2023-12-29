use std::fs;
use clap::Parser;
use aoc2023_day_12::{Cli, solve_part1};

fn main() {
    let args: Cli = Cli::parse();
    let input: String = fs::read_to_string(args.data_dir.join("input.txt")).unwrap();
    println!("{}", solve_part1(&input));
}