#[macro_use]
extern crate lazy_static;
extern crate regex;
use regex::Regex;

use std::fs::File;
use std::io::{BufReader, Lines};

type Shape = [i32; 3];

fn parse_line(line: &str) -> Result<Shape, String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)\s+(\d+)\s+(\d+)$").unwrap();
    }
    let cap = RE.captures(line).ok_or_else(|| String::from("Bad input format"))?;
    let extract = |n| cap.at(n).unwrap().parse().unwrap();
    Ok([extract(1), extract(2), extract(3)])
}

fn triangle(shape: &Shape) -> bool {
    let max: i32 = shape.iter().cloned().max().unwrap_or(0);
    let sum: i32 = shape.iter().sum();
    sum > 2 * max
}

fn read_file_lines() -> Lines<BufReader<File>> {
    use std::env;
    use std::io::BufRead;
    let filename = env::args().nth(1).expect("You forgot to pass the input file dude");
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    BufRead::lines(reader)
}

fn solve1() -> usize {
    read_file_lines()
        .map(|s| parse_line(&s.unwrap()).unwrap())
        .filter(triangle)
        .count()
}

fn main() {
    println!("{}", solve1());
}

#[cfg(test)]
mod tests {
    use super::{parse_line, triangle};

    #[test]
    fn parse() {
        assert_eq!(parse_line(" 1 2 34").unwrap(), [1, 2, 34]);
    }

    #[test]
    fn check() {
        assert!(triangle(&[3,4,5]));
        assert!(!triangle(&[5,10,25]));
    }
}
