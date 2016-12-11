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
use std::collections::HashSet;

type Step = (char, i32);
type Loc = (i32, i32);

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

fn solve1(steps: &[Step]) -> i32 {
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

fn mov((x, y): Loc, (dx, dy): Loc) -> Loc {
    (x + dx, y + dy)
}

fn solve2(steps: &[Step]) -> Option<i32> {
    let mut dir = 0;
    let mut loc: Loc = (0, 0);
    let mut seen = HashSet::new();
    seen.insert(loc);

    for &(turn, walk) in steps.iter() {
        let offset = match turn {
            'L' => -1,
            'R' => 1,
            _   => unreachable!("bad turn"),
        };
        dir = modulo(dir + offset, 4);
        let step = match dir {
            0 => (0, 1),
            1 => (1, 0),
            2 => (0, -1),
            3 => (-1, 0),
            _ => unreachable!("bad direction"),
        };
        for _ in 0..walk {
            loc = mov(loc, step);
            if seen.contains(&loc) {
                return Some(loc.0.abs() + loc.1.abs());
            }
            seen.insert(loc);
        }
    }
    None
}

fn modulo<T: Rem<T, Output=T> + Add<T, Output=T> + Copy>(a: T, b: T) -> T {
    ((a % b) + b) % b
}

fn read_file_param() -> Vec<u8> {
    let filename = env::args().nth(1).expect("You forgot to pass the input file dude");
    let mut content = Vec::new();
    File::open(filename).unwrap().read_to_end(&mut content).unwrap();
    content
}

fn main() {
    let input = steps(&read_file_param()).to_result().unwrap();
    println!("Part 1: {:?}", solve1(&input));
    println!("Part 2: {:?}", solve2(&input));
}

#[cfg(test)]
mod tests {
    use super::{step, steps, solve1, solve2};
    use std::fmt::Debug;

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

    fn full_test<T: PartialEq<T> + Debug, F: FnOnce(&[(char, i32)]) -> T>(solve: F, input: &[u8], expected: T) {
        let parsed = steps(input).to_result().unwrap();
        let result = solve(&parsed);
        assert_eq!(expected, result)
    }

    #[test]
    fn example1() {
        full_test(solve1, b"R2, L3", 5)
    }

    #[test]
    fn example2() {
        full_test(solve1, b"R2, R2, R2", 2)
    }

    #[test]
    fn example3() {
        full_test(solve1, b"R5, L5, R5, R3", 12)
    }

    #[test]
    fn example4() {
        full_test(solve2, b"R8, R4, R4, R8", Some(4))
    }
}
