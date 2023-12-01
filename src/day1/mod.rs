use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::ops::Index;
use std::path::Path;
use lazy_static::lazy_static;


lazy_static! {
    static ref BACKWARDS_CHAR_MAP: HashMap<char, Vec<String>> = {
        let mut m = HashMap::new();
        m.insert('o', vec![String::from("orez"), String::from("owt")]);
        m.insert('e', vec![String::from("eno"), String::from("eerht"), String::from("evif"), String::from("enin")]);
        m.insert('r', vec![String::from("ruof")]);
        m.insert('x', vec![String::from("xis")]);
        m.insert('n', vec![String::from("neves")]);
        m.insert('t', vec![String::from("thgie")]);

        m
    };

    static ref BACKWARDS_DIGIT_MAP: HashMap<String, i32> = {
        let mut m = HashMap::new();
        m.insert(String::from("orez"), 0);
        m.insert(String::from("eno"), 1);
        m.insert(String::from("owt"), 2);
        m.insert(String::from("eerht"), 3);
        m.insert(String::from("ruof"), 4);
        m.insert(String::from("evif"), 5);
        m.insert(String::from("xis"), 6);
        m.insert(String::from("neves"), 7);
        m.insert(String::from("thgie"), 8);
        m.insert(String::from("enin"), 9);

        m
    };

    static ref CHAR_MAP: HashMap<char, Vec<String>> = {
        let mut m = HashMap::new();
        m.insert('z', vec![String::from("zero")]);
        m.insert('o', vec![String::from("one")]);
        m.insert('t', vec![String::from("two"), String::from("three")]);
        m.insert('f', vec![String::from("four"), String::from("five")]);
        m.insert('s', vec![String::from("six"), String::from("seven")]);
        m.insert('e', vec![String::from("eight")]);
        m.insert('n', vec![String::from("nine")]);

        m
    };

    static ref DIGIT_MAP: HashMap<String, i32> = {
        let mut m = HashMap::new();
        m.insert(String::from("zero"), 0);
        m.insert(String::from("one"), 1);
        m.insert(String::from("two"), 2);
        m.insert(String::from("three"), 3);
        m.insert(String::from("four"), 4);
        m.insert(String::from("five"), 5);
        m.insert(String::from("six"), 6);
        m.insert(String::from("seven"), 7);
        m.insert(String::from("eight"), 8);
        m.insert(String::from("nine"), 9);

        m
    };
}

pub fn solve() {
    let mut part1result = 0;
    let mut part2result = 0;
    if let Ok(lines) = read_lines("resources/day_1_input.txt") {
        for line in lines {
            if let Ok(line) = line {
                part1result += get_calibration_value(&line);
                part2result += get_part2_calibration_value(&line);
            }
        }
    }

    println!("Solution for part 1 is {part1result}");
    println!("Solution for part 2 is {part2result}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_calibration_value(line: &str) -> i32
{
    let first_digit = line.chars().find(|&c| c.is_digit(10)).unwrap();
    let last_digit = line.chars().rev().find(|&c| c.is_digit(10)).unwrap();

    return (first_digit as i32 - '0' as i32) * 10 + last_digit as i32 - '0' as i32;
}

fn get_part2_calibration_value(line: &str) -> i32
{
    let first_digit = find_digit(line.chars(), &BACKWARDS_CHAR_MAP, &BACKWARDS_DIGIT_MAP);
    let last_digit = find_digit(line.chars().rev(), &CHAR_MAP, &DIGIT_MAP);

    first_digit * 10 + last_digit
}

fn find_digit<I>(iter: I, char_map: &HashMap<char, Vec<String>>, digit_map: &HashMap<String, i32>) -> i32
where I : IntoIterator<Item=char>
{
    let mut window = BoundedQueue::new(5);
    for c in iter {
        if let Some(digit) = c.to_digit(10) {
            return digit as i32;
        }

        window.push(c);
        if let Some(digits) = char_map.get(&c) {
            for digit in digits {
                if window.matches(digit) {
                    return *digit_map.get(digit).unwrap()
                }
            }
        }
    }

    eprintln!("No digit found");
    0
}

struct BoundedQueue {
    deque: VecDeque<char>,
    capacity: usize,
}

impl BoundedQueue {
    fn new(capacity: usize) -> Self {
        BoundedQueue {
            deque: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    fn push(&mut self, item: char) {
        if self.deque.len() == self.capacity {
            self.deque.pop_front();
        }
        self.deque.push_back(item);
    }

    fn matches(&self, pattern: &str) -> bool {
        if self.deque.len() < pattern.len() {
            return false;
        }

        for (i, ch) in pattern.chars().enumerate() {
            if let Some(window_ch) = self.deque.iter().rev().nth(i) {
                if *window_ch != ch {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }
}

impl Index<usize> for BoundedQueue {
    type Output = char;

    fn index(&self, index: usize) -> &Self::Output {
        &self.deque[index]
    }
}
