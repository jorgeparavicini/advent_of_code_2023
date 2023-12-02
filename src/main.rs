use std::env;
use std::time::Instant;
use crate::solver::Solver;

mod day1;
mod day2;
mod solver;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: advent_of_code_2023 <day>");
        std::process::exit(1);
    }

    let solver: Box<dyn Solver> = match args[1].as_str() {
        "1" => Box::new(day1::Day1Solver),
        "2" => Box::new(day2::Day2Solver),
        _ => panic!("Day not implemented")
    };

    let start = Instant::now();
    solver.solve();
    let duration = start.elapsed();

    println!("Solved in {:?}", duration);
}
