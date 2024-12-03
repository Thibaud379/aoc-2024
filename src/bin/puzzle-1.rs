use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Not enough arguments\nUSAGE: PART ./puzzle-1.exe FILE\n\tWhere PART is one of `1` or `2`");
        return;
    }
    let Ok(lines) = File::open(args[2].clone()).map(|f| BufReader::new(f).lines()) else {
        eprintln!("Error reading `{}`", args[2]);
        return;
    };
    let sum: u64 = match args[1].as_str() {
        "1" => part1(lines),
        "2" => part2(lines),
        _ => {
            eprint!("Arguments invalid\nUSAGE: PART ./puzzle-1.exe FILE\n\tWhere PART must be one of `1` or `2`");
            return;
        }
    };
    println!("Got result `{sum}`!");
}

fn part1(lines: std::io::Lines<BufReader<File>>) -> u64 {
    let (mut left, mut right) = lines
        .map(|l| {
            l.unwrap()
                .split_ascii_whitespace()
                .map(str::parse)
                .map(Result::unwrap)
                .collect::<Vec<u64>>()
        })
        .fold((vec![], vec![]), |mut acc, v| {
            acc.0.push(v[0]);
            acc.1.push(v[1]);
            acc
        });
    left.sort();
    right.sort();
    left.into_iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum()
}

fn part2(lines: std::io::Lines<BufReader<File>>) -> u64 {
    let (left, mut right) = lines
        .map(|l| {
            l.unwrap()
                .split_ascii_whitespace()
                .map(str::parse)
                .map(Result::unwrap)
                .collect::<Vec<u64>>()
        })
        .fold((vec![], HashMap::new()), |mut acc, v| {
            acc.0.push(v[0]);
            acc.1
                .entry(v[1])
                .and_modify(|va| *va += 1u64)
                .or_insert(1u64);
            acc
        });
    left.into_iter()
        .map(|id| id * *right.entry(id).or_default())
        .sum()
}
