extern crate md5;
use md5::Context;

use std::iter::Iterator;
use std::io::Write;

fn starts_with_20_zeros(digest: &[u8; 16]) -> bool {
    digest[0] as u32 + digest[1] as u32 + (digest[2] >> 4) as u32 == 0
}

fn sixth_hex(digest: &[u8; 16]) -> u8 {
    digest[2] & 0x0f
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

fn solve(input: &[u8], limit: usize) -> String {
    let base = {
        let mut init = Context::new();
        init.write_all(input).unwrap();
        init
    };
    let mut full = Context::new();
    (0..).filter_map(|n| {
        full.clone_from(&base);
        full.write_all(n.to_string().as_bytes()).unwrap();
        let digest = full.compute();
        if starts_with_20_zeros(&digest) {
            Some(sixth_hex(&digest))
        } else {
            None
        }
    }).take(limit).map(to_hex).collect()
}

fn main() {
    println!("{}", solve(b"cxdnnyjw", 8));
}

#[cfg(test)]
mod tests {
    use super::{solve, starts_with_20_zeros};
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
    fn example() {
        assert_eq!(&solve(b"abc", 8), "18f47a30")
    }
}
