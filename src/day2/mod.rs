use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use crate::solver::Solver;

pub struct Day2Solver;

impl Solver for Day2Solver {
    fn solve(&self) {
        let file = File::open("resources/day_2_input.txt").unwrap();
        let lines: io::Result<io::Lines<BufReader<File>>> = Ok(BufReader::new(file).lines());
        let mut result = 0;
        let mut result2 = 0;

        for (id, line) in lines.unwrap().enumerate() {
            let color = Color::create_from_line(&line.unwrap(), id);
            if color.is_valid_for_part_1() {
                result += id + 1;
            }
            result2 += color.red * color.blue * color.green;
        }

        println!("Solution for part 1 is {result}");
        println!("Solution for part 2 is {result2}");
    }
}

struct Color {
    red: u32,
    green: u32,
    blue: u32,
}

fn read_number(iter: &mut std::str::Chars) -> u32 {
    let mut size = 0;

    loop {
        if let Some(c) = iter.next() {
            if let Some(digit) = c.to_digit(10) {
                size *= 10;
                size += digit;
            } else {
                break;
            }
        }
    }

    size
}

impl Color {
    fn create() -> Self {
        Color {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn create_from_line(line: &str, line_index: usize) -> Color {
        let mut color = Color::create();
        let mut iter = line.chars();
        // Advance iter to start of first item
        let nr_to_skip = (line_index as f64 + 1.0f64).log10().floor() as usize + 7;
        iter.nth(nr_to_skip);

        loop {
            let nr = read_number(&mut iter);
            let start_char = iter.next().unwrap();
            if match start_char {
                'r' => {
                    if nr > color.red {
                        color.red = nr;
                    }
                    iter.nth(3)
                }
                'g' => {
                    if nr > color.green {
                        color.green = nr;
                    }
                    iter.nth(5)
                }
                'b' => {
                    if nr > color.blue {
                        color.blue = nr;
                    }
                    iter.nth(4)
                }
                _ => panic!()
            } == None {
                break;
            }
        }

        color
    }

    fn is_valid_for_part_1(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
}
