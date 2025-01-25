#![allow(dead_code)]
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    convert::identity,
    env,
    fs::File as FileFs,
    hash::RandomState,
    io::Read,
    iter,
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
    combinations: HashMap<String, usize>,
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
    let combinations = HashMap::new();
    let mut d = PuzzleData {
        combinations,
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
        self.combinations(design, 0, false) != 0
    }

    fn combinations(&mut self, design: &str, depth: usize, debug: bool) -> usize {
        let indent = " ".repeat(depth);
        let s = design.to_string();
        if let Some(c) = self.combinations.get(design) {
            if debug {
                println!("{indent}FOUND {design}={c}");
            }
            return *c;
        };
        let mut c = 0;
        if self.patterns.contains(&s) {
            c += 1;
        } else {
            if design.len() == 0 {
                return 1;
            }
            if design.len() == 1 {
                self.combinations.insert(design.to_string(), 0);
                return 0;
            }
        }
        let pats: Vec<String> = self
            .patterns
            .iter()
            .filter(|p| p.len() < design.len() && design.starts_with(p.as_str()))
            .cloned()
            .collect();
        for p in pats {
            if debug {
                println!("{indent} PAT {p} IN {design}")
            }
            c += self.combinations(&design[p.len()..], depth + 1, debug);
        }
        self.combinations.insert(design.to_string(), c);
        c
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
    println!("Processed input");
    let mut res = 0;
    let possible: Vec<String> = designs
        .into_iter()
        .filter(|d| data.is_possible(d))
        .collect();

    for d in possible {
        print!("{d} =>");
        let h = data.combinations(d.as_str(), 0, false);
        println!(" {}", h);
        res += h;
    }

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

        assert_eq!(part1(data), 6)
    }
    #[test]
    fn test_2() {
        let mut data = parse_input(examples::EX_1);
        println!("{:?}", data.combinations("bgrg", 0, false));
        assert!(matches!(part2(data), 16));
    }
}
