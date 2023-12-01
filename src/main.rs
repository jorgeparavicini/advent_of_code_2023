use std::env;

mod day1;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: advent_of_code_2023 <day>");
        std::process::exit(1);
    }

    match args[1].as_str() {
        "1" => day1::solve(),
        _ => eprintln!("Day not implemented")
    }
}
