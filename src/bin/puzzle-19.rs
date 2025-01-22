#![allow(dead_code)]
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    convert::identity,
    env,
    fs::File as FileFs,
    hash::RandomState,
    io::Read,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Not enough arguments\nUSAGE: PART ./puzzle-19.exe FILE\n\tWhere PART is one of `1` or `2`");
        return;
    }
    let mut raw_data = String::new();
    let Ok(mut file) = FileFs::open(args[2].clone()) else {
        eprintln!("Error reading `{}`", args[2]);
        return;
    };
    file.read_to_string(&mut raw_data).unwrap();

    let data = parse_input(&raw_data);
    match args[1].as_str() {
        "1" => {
            part1(data);
        }
        "2" => {
            part2(data);
        }
        _ => {
            eprint!("Arguments invalid\nUSAGE: PART ./puzzle-19.exe FILE\n\tWhere PART must be one of `1` or `2`");
            return;
        }
    };
}

type PuzzleResult = usize;

#[derive(Clone, Debug)]
struct PuzzleData {
    patterns: Vec<String>,
    designs: Vec<String>,
    seen_designs: HashMap<String, usize>,
}

fn parse_input(data: &str) -> PuzzleData {
    let mut lines = data.lines();
    let patterns: Vec<String> = lines
        .next()
        .unwrap()
        .split(',')
        .map(str::trim)
        .map(str::to_owned)
        .collect();
    lines.next();
    let designs = lines.map(str::to_owned).collect();
    let seen_designs = HashMap::new();
    let mut d = PuzzleData {
        seen_designs,
        patterns: patterns.clone(),
        designs,
    };

    for p in patterns {
        d.combinations(&p, 0, false);
    }
    d
}

impl PuzzleData {
    fn is_possible(&mut self, design: &str) -> bool {
        let s = design.to_string();
        if self.patterns.contains(&s) {
            return true;
        }
        if design.len() == 1 {
            return false;
        }
        if let Some(v) = self.seen_designs.get(design) {
            return *v == 1;
        }
        if !self.patterns.iter().any(|p| design.starts_with(p)) {
            self.seen_designs.insert(s, 0);
            return false;
        }
        let res = (1..design.len())
            .any(|i| self.is_possible(&design[..i]) && self.is_possible(&design[i..]));
        if res {
            self.seen_designs.insert(s, 1);
        }
        res
    }

    fn combinations(&mut self, design: &str, depth: usize, debug: bool) -> usize {
        let indent = "\t".repeat(depth);
        if debug {
            println!("{indent}{design}");
        }
        let s = design.to_string();
        let mut res: usize = 0;
        if self.patterns.contains(&s) {
            res += 1;
        }
        if design.len() == 1 {
            if debug {
                println!("{indent}LEN {res}");
            }
            return res;
        }
        if let Some(v) = self.seen_designs.get(design) {
            if debug {
                println!("{indent}PREV {v}");
            }
            return *v;
        }

        if !self.patterns.iter().any(|p| design.starts_with(p)) {
            self.seen_designs.insert(s, 0);
            if debug {
                println!("{indent}START 0");
            }
            return 0;
        }

        res += (1..design.len())
            .map(|i| {
                self.combinations(&design[..i], depth + 1, debug)
                    * self.combinations(&design[i..], depth + 1, debug)
            })
            .max()
            .unwrap();
        if res > 0 {
            self.seen_designs.insert(s, res);
        }

        if debug {
            println!("{indent}{design}({res})");
        }
        res
    }
}

fn part1(mut data: PuzzleData) -> PuzzleResult {
    let designs = data.designs.clone();
    let res = designs
        .iter()
        .filter(|d| data.is_possible(d.as_str()))
        .count();
    println!("{res}");
    res
}
fn part2(mut data: PuzzleData) -> PuzzleResult {
    let designs = data.designs.clone();
    for d in &designs {
        println!("{d} => {}", data.combinations(d, 0, true));
    }
    let res = designs
        .iter()
        .map(|d| data.combinations(d.as_str(), 0, false))
        .sum();
    println!("{res}");
    res
}

#[cfg(test)]
mod tests {
    use crate::*;
    mod examples {
        pub const EX_1: &'static str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
    }

    #[test]
    fn test_1() {
        let mut data = parse_input(examples::EX_1);

        assert!(matches!(part1(data), 6))
    }
    #[test]
    fn test_2() {
        let mut data = parse_input(examples::EX_1);
        println!("{}\n", data.combinations("bgb", 0, true));
        println!("{}\n", data.combinations("gbr", 0, true));
        println!("{}\n", data.combinations("bgbr", 0, true));
        // assert!(matches!(part2(data), 16));
    }
}
