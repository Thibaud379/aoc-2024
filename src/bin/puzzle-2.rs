use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Not enough arguments\nUSAGE: PART ./puzzle-2.exe FILE\n\tWhere PART is one of `1` or `2`");
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
            eprint!("Arguments invalid\nUSAGE: PART ./puzzle-2.exe FILE\n\tWhere PART must be one of `1` or `2`");
            return;
        }
    };
    println!("Got result `{sum}`!");
}

fn check_levels<T: Iterator<Item = u64>>(levels: T) -> bool {
    let mut prev = None;
    let mut dir = None;
    for level in levels {
        if let Some(p) = prev {
            if !(1..=3).contains(&level.abs_diff(p)) {
                return false;
            }
            if dir.is_none() {
                dir = Some(level < p);
            } else if dir.is_some_and(|d| d != (level < p)) {
                return false;
            }
        }
        prev = Some(level);
    }
    true
}
fn part1(lines: std::io::Lines<BufReader<File>>) -> u64 {
    lines
        .map(|l| {
            check_levels(
                l.unwrap()
                    .split_ascii_whitespace()
                    .map(str::parse::<u64>)
                    .map(Result::unwrap),
            )
        })
        .filter(|b| *b)
        .count() as u64
}

fn part2(lines: std::io::Lines<BufReader<File>>) -> u64 {
    lines
        .map(|l| {
            let levels = l
                .unwrap()
                .split_ascii_whitespace()
                .map(str::parse::<u64>)
                .map(Result::unwrap)
                .collect::<Vec<_>>();
            if check_levels(levels.iter().copied()) {
                return true;
            }
            let vars = vec![levels.clone(); levels.len()];
            vars.into_iter().enumerate().any(|(id, mut var)| {
                var.remove(id);
                check_levels(var.into_iter())
            })
        })
        .filter(|b| *b)
        .count() as u64
}
