#[macro_use]
extern crate nom;
use nom::digit;
use std::str;
use std::str::FromStr;
use std::fmt::Debug;
use std::env;
use std::fs::File;
use std::io::Read;
use std::ops::{Rem, Add};

type Step = (char, i32);

fn force_parse<T: FromStr>(input: &[u8]) -> T 
        where <T as FromStr>::Err: Debug {
    str::from_utf8(input)
        .unwrap()
        .parse()
        .unwrap()
}

named!(step(&[u8]) -> Step,
    tuple!(
        alt!(char!('R') | char!('L')),
        map!(digit, force_parse)
    )
);

named!(steps(&[u8]) -> Vec<Step>,
    separated_list!(
        tag!(", "),
        step
    )
);

fn solve(steps: Vec<Step>) -> i32 {
    let mut dir = 0;
    let mut x = 0;
    let mut y = 0;

    for &(turn, walk) in steps.iter() {
        let offset = match turn {
            'L' => -1,
            'R' => 1,
            _   => unreachable!("bad turn"),
        };
        dir = modulo(dir + offset, 4);
        match dir {
            0 => y += walk,
            1 => x += walk,
            2 => y -= walk,
            3 => x -= walk,
            _ => unreachable!("bad direction"),
        };
    }
    x.abs() + y.abs()
}

fn modulo<T: Rem<T, Output=T> + Add<T, Output=T> + Copy>(a: T, b: T) -> T {
    ((a % b) + b) % b
}

fn read_file_param() -> Vec<u8> {
    let filename = env::args().nth(1).unwrap();
    let mut content = Vec::new();
    File::open(filename).unwrap().read_to_end(&mut content).unwrap();
    content
}

fn main() {
    println!("{}", solve(steps(&read_file_param()).to_result().unwrap()));
}

#[cfg(test)]
mod tests {
    use super::{step, steps, solve};

    #[test]
    fn parse_step() {
        let result = step(b"R12").to_result().unwrap();
        let expected = ('R', 12);
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_steps() {
        let result = steps(b"L12, R2, R1").to_result().unwrap();
        let expected = vec![('L',12),('R',2),('R',1)];
        assert_eq!(result, expected);
    }

    fn full_test(input: &[u8], expected: i32) {
        let parsed = steps(input).to_result().unwrap();
        let result = solve(parsed);
        assert_eq!(expected, result)
    }

    #[test]
    fn example1() {
        full_test(b"R2, L3", 5)
    }

    #[test]
    fn example2() {
        full_test(b"R2, R2, R2", 2)
    }

    #[test]
    fn example3() {
        full_test(b"R5, L5, R5, R3", 12)
    }

}
