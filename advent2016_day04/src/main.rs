#[macro_use]
extern crate lazy_static;
extern crate regex;
use regex::Regex;

use std::fs::File;
use std::io::{BufReader, Lines};
use std::collections::HashMap;

fn parse_line<'a>(line: &'a str) -> (&'a str, &'a str, i32) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(.*)-(\d+)\[(.*)\]$").unwrap();
    }
    let cap = RE.captures(line).expect("Bad input format");
    (
        cap.at(1).unwrap(),
        cap.at(3).unwrap(),
        cap.at(2).unwrap().parse().unwrap()
    )
}

fn read_file_lines() -> Lines<BufReader<File>> {
    use std::env;
    use std::io::BufRead;
    let filename = env::args().nth(1).expect("You forgot to pass the input file dude");
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    BufRead::lines(reader)
}

fn checksum(items: &str) -> String {
    use std::cmp::Ordering;
    let mut counts = HashMap::<char, u64>::new();
    for c in items.chars() {
        if c != '-' {
            let counter = counts.entry(c).or_insert(0);
            *counter += 1;
        }
    }
    let mut counts: Vec<_> = counts.iter().collect();
    counts.sort_by(|&(c1, f1), &(c2, f2)| {
        let freq = f1.cmp(&f2);
        if freq != Ordering::Equal {
            freq.reverse()
        } else {
            c1.cmp(&c2)
        }
    });
    counts.iter().take(5).map(|t| t.0).cloned().collect()
}

fn solve1() -> i32 {
    read_file_lines()
        .filter_map(|s| {
            let line = s.unwrap();
            let (code, chk, id) = parse_line(&line);
            if &checksum(code) == chk {
                Some(id)
            } else {
                None
            }
        })
        .sum()
}

fn main() {
    println!("{}", solve1());
}

#[cfg(test)]
mod tests {
    use super::{parse_line, checksum};

    #[test]
    fn parse() {
        assert_eq!(
            parse_line("aaaaa-bbb-z-y-x-123[abxyz]"),
            ("aaaaa-bbb-z-y-x", "abxyz", 123));
    }

    #[test]
    fn matching() {
        assert_eq!(&checksum("aaaaa-bbb-z-y-x"), "abxyz");
    }
}