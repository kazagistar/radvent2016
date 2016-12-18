extern crate md5;
use md5::{Context, Digest};

use std::iter::Iterator;
use std::io::Write;

#[inline]
fn starts_with_20_zeros(digest: &[u8; 16]) -> bool {
    digest[0] as u32 + digest[1] as u32 + (digest[2] >> 4) as u32 == 0
}

fn sixth_hex(digest: &[u8; 16]) -> u8 {
    digest[2] & 0x0f
}

fn seventh_hex(digest: &[u8; 16]) -> u8 {
    digest[3] >> 4
}

fn to_hex(input: u8) -> char {
    // the things we do for optimizations
    match input {
        0 => '0',
        1 => '1',
        2 => '2',
        3 => '3',
        4 => '4',
        5 => '5',
        6 => '6',
        7 => '7',
        8 => '8',
        9 => '9',
        10 => 'a',
        11 => 'b',
        12 => 'c',
        13 => 'd',
        14 => 'e',
        15 => 'f',
        _ => 'X',
    }
}

struct Digested {
    iter: u64,
    base: Context,
}

impl Digested {
    fn from(input: &[u8]) -> Self {
        Digested {
            iter: 0,
            base: {
                let mut init = Context::new();
                init.write_all(input).unwrap();
                init
            },
        }
    }
}

impl Iterator for Digested {
    type Item = Digest;

    #[inline]
    fn next(&mut self) -> Option<Digest> {
        let mut full = self.base.clone();
        full.write_all(self.iter.to_string().as_bytes()).unwrap();
        self.iter += 1;
        Some(full.compute())
    }
}

fn solve1(input: &[u8], limit: usize) -> String {
    Digested::from(input)
        .filter(|d| starts_with_20_zeros(&d))
        .map(|d| sixth_hex(&d))
        .take(limit)
        .map(to_hex)
        .collect()
}

const EMPTY: u8 = 255;

fn solve2(input: &[u8], limit: u8) -> String {
    let mut buffer: Vec<u8> = (0..limit).map(|_| EMPTY).collect();
    for digest in Digested::from(input).filter(|d| starts_with_20_zeros(&d)) {
        let index = sixth_hex(&digest);
        if index < limit && buffer[index as usize] == EMPTY {
            let value = seventh_hex(&digest);
            // println!("placed {} at {}", index, value);
            buffer[index as usize] = value;
            if buffer.iter().cloned().all(|cell| cell != EMPTY) {
                break;
            }
        }
    }
    buffer.iter().cloned().map(to_hex).collect()
}

fn main() {
    let input = b"cxdnnyjw";
    println!("Part 1: {}", solve1(input, 8));
    println!("Part 2: {}", solve2(input, 8));
}

#[cfg(test)]
mod tests {
    use super::{solve1, solve2, starts_with_20_zeros};
    use md5::Context;
    use std::io::Write;

    #[test]
    fn partial_reuse() {
        let base = {
            let mut init = Context::new();
            init.write_all(b"abc").unwrap();
            init
        };
        let mut temp = base.clone();
        temp.write_all(b"3231929").unwrap();
        assert!(starts_with_20_zeros(&temp.compute()));
    }

    #[test]
    fn example1() {
        assert_eq!(&solve1(b"abc", 8), "18f47a30")
    }

    #[test]
    fn example2() {
        assert_eq!(&solve2(b"abc", 8), "05ace8e3")
    }
}
