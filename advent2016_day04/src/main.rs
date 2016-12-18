#[macro_use]
extern crate lazy_static;
extern crate regex;
use regex::Regex;

use std::fs::File;
use std::io::{BufReader, Lines};
use std::collections::HashMap;
use std::char;

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

fn rotate(c: char, n: u32) -> char {
    let offset = 'a' as u32;
    match c {
        '-' => ' ',
        'a' ... 'z' => {
            let c = (c as u32) - offset;
            let rotated = (c + n) % 26 + offset;
            char::from_u32(rotated).unwrap()
        },
        _ => c,
    }
}

fn rotate_all(s: &str, n: u32) -> String {
    s.chars().map(|c| rotate(c, n)).collect()
}

fn solve() -> i32 {
    read_file_lines()
        .filter_map(|s| {
            let line = s.unwrap();
            let (code, chk, id) = parse_line(&line);
            if &checksum(code) == chk {
                let decoded = rotate_all(&code, id as u32);
                if decoded.contains("pole") {
                    println!("{}: {}", id, decoded);
                }
                Some(id)
            } else {
                None
            }
        })
        .sum()
}

fn main() {
    println!("TOTAL: {}", solve());
}

#[cfg(test)]
mod tests {
    use super::{parse_line, checksum, rotate, rotate_all};

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

    #[test]
    fn rotations() {
        assert_eq!(rotate('a',1), 'b');
        assert_eq!(&rotate_all("qzmt-zixmtkozy-ivhz", 343), "very encrypted name")
    }
}
