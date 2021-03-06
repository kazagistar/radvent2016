type Key = char;
type Pos = (i8, i8);

const KEYPAD1: [[Key; 3]; 3] = [
    ['1','2','3'],
    ['4','5','6'],
    ['7','8','9']];

fn keypad1((x, y): Pos) -> Option<Key> {
    KEYPAD1
        .get(y as usize)
        .and_then(|row| row.get(x as usize))
        .map(|c| c.clone())
}

const KEYPAD2: [[Key; 5]; 5] = [
    ['0','0','1','0','0'],
    ['0','2','3','4','0'],
    ['5','6','7','8','9'],
    ['0','A','B','C','0'],
    ['0','0','D','0','0']];

fn keypad2((x, y): Pos) -> Option<Key> {
    KEYPAD2
        .get(y as usize)
        .and_then(|row| row.get(x as usize))
        .and_then(|&c| {
            if c != '0' {
                Some(c)
            } else {
                None
            }
        })
}

fn mov(dir: char, (x, y): Pos) -> Pos {
    match dir {
        'L' => (x-1, y),
        'R' => (x+1, y),
        'U' => (x, y-1),
        'D' => (x, y+1),
        _   => (x, y),
    }
}

fn solve<F: Fn(Pos) -> Option<Key>>(start: Pos, keypad: F, path: &str) -> String {
    let mut pos = start;
    let mut result = String::new();
    for step in path.chars() {
        if step == '\n' {
            result.push(keypad(pos).unwrap());
        } else {
            let next = mov(step, pos);
            if let Some(_) = keypad(next) {
                pos = next;
            }
        }
    }
    result
}

fn solve1(path: &str) -> String {
    solve((1,1), keypad1, path)
}

fn solve2(path: &str) -> String {
    solve((0,2), keypad2, path)
}

fn read_file_param() -> String {
    use std::env;
    use std::fs::File;
    use std::io::Read;
    let filename = env::args().nth(1).expect("You forgot to pass the input file dude");
    let mut content = String::new();
    File::open(filename).unwrap().read_to_string(&mut content).unwrap();
    content
}

fn main() {
    let input = read_file_param();
    println!("Part 1: {}", solve1(&input));
    println!("Part 2: {}", solve2(&input));
}

#[cfg(test)]
mod tests {
    use super::{keypad1, solve1, solve2};

    #[test]
    fn translation() {
        assert_eq!(keypad1((1,2)), Some('8'));
        assert_eq!(keypad1((3,2)), None);
    }

    const EXAMPLE: &'static str = "ULL\nRRDDD\nLURDL\nUUUUD\n";

    #[test]
    fn example1() {
        assert_eq!(&solve1(EXAMPLE), "1985");
    }

    #[test]
    fn example2() {
        assert_eq!(&solve2(EXAMPLE), "5DB3");
    }
}
