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

fn get_triangles() -> Vec<Shape> {
    read_file_lines().map(|s| parse_line(&s.unwrap()).unwrap()).collect()
}

fn solve1(input: &[Shape]) -> usize {
    input.iter().filter(|s| triangle(&s)).count()
}

fn solve2(input: &[Shape]) -> usize {
    input
        .chunks(3)
        .flat_map(|shapes| {
            shapes[0].iter()
                .zip(shapes[1].iter())
                .zip(shapes[2].iter())
                .map(|((&a, &b), &c)| [a, b, c])
        })
        .filter(triangle)
        .count()
}

fn main() {
    let input = get_triangles();
    println!("Part 1: {}", solve1(&input));
    println!("Part 2: {}", solve2(&input));
}

#[cfg(test)]
mod tests {
    use super::{parse_line, triangle, solve1, solve2};

    #[test]
    fn parse() {
        assert_eq!(parse_line(" 1 2 34").unwrap(), [1, 2, 34]);
    }

    #[test]
    fn check() {
        assert!(triangle(&[3,4,5]));
        assert!(!triangle(&[5,10,25]));
    }

    #[test]
    fn example() {
        let input = [
            [101, 301, 501],
            [102, 302, 502],
            [103, 303, 503],
            [201, 401, 601],
            [202, 402, 602],
            [203, 403, 603]];
        assert_eq!(solve1(&input), 3);
        assert_eq!(solve2(&input), 6);
    }
}
