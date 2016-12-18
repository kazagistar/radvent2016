use std::fs::File;
use std::io::{BufReader, Lines};
use std::char;

#[derive(Default)]
struct LetterCounter {
    counts: [u32; 26]
}

impl LetterCounter {
    fn new() -> Self {
        LetterCounter {
            counts: [0; 26]
        }
    }

    fn inc(&mut self, c: char) {
        let index = (c as u32) - ('a' as u32);
        self.counts[index as usize] += 1;
    }

    fn max(&self) -> char {
        let (index, _) = self.counts.iter().enumerate().max_by_key(|t| t.1).unwrap();
        char::from_u32(index as u32 + ('a' as u32)).unwrap()
    }
}

struct FreqCounter {
    data: Vec<LetterCounter>
}

impl FreqCounter {
    fn from_first_line(first: &str) -> FreqCounter {
        FreqCounter {
            data: first.chars().map(|c| {
                    let mut init = LetterCounter::new();
                    init.inc(c);
                    init
                }).collect()
        }
    }

    fn process(&mut self, line: &str) {
        for (i, c) in line.chars().enumerate() {
            self.data[i].inc(c);
        }
    }

    fn maximums(&self) -> String {
        self.data.iter().map(LetterCounter::max).collect()
    }
}

fn read_file_lines() -> Lines<BufReader<File>> {
    use std::env;
    use std::io::BufRead;
    let filename = env::args().nth(1).expect("You forgot to pass the input file dude");
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    BufRead::lines(reader)
}

fn solve() -> String {
    let mut lines = read_file_lines();
    let mut counter = FreqCounter::from_first_line(&lines.next().unwrap().unwrap());
    for line in lines.map(|n| n.unwrap()) {
        counter.process(&line);
    }
    counter.maximums()
}

fn main() {
    println!("{}", solve());
}

#[cfg(test)]
mod tests {
    use super::FreqCounter;

    #[test]
    fn example() {
        let input = &[
            "eedadn",
            "drvtee",
            "eandsr",
            "raavrd",
            "atevrs",
            "tsrnev",
            "sdttsa",
            "rasrtv",
            "nssdts",
            "ntnada",
            "svetve",
            "tesnvt",
            "vntsnd",
            "vrdear",
            "dvrsen",
            "enarar",
        ];
        let mut lines = input.iter();
        let mut counter = FreqCounter::from_first_line(lines.next().unwrap());
        for line in lines {
            counter.process(&line);
        }
        let chars: String = counter.maximums();
        assert_eq!(&chars, "easter");
    }
}
