use std::{
    collections::HashSet,
    env,
    fs::File,
    io::{BufRead, BufReader},
    iter,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Not enough arguments\nUSAGE: PART ./puzzle-7.exe FILE\n\tWhere PART is one of `1` or `2`");
        return;
    }
    let Ok(lines) = File::open(args[2].clone()).map(|f| BufReader::new(f).lines()) else {
        eprintln!("Error reading `{}`", args[2]);
        return;
    };
    let Ok(data) = parse_input(lines) else {
        eprintln!("Error reading `{}`", args[2]);
        return;
    };
    let sum: u64 = match args[1].as_str() {
        "1" => part1(data),
        "2" => part2(data),
        _ => {
            eprint!("Arguments invalid\nUSAGE: PART ./puzzle-7.exe FILE\n\tWhere PART must be one of `1` or `2`");
            return;
        }
    };
    println!("Got result `{sum}`!");
}

#[derive(Clone, Debug)]
struct PuzzleData {
    eqs: Vec<(u64, Vec<u64>)>,
}

fn parse_input(lines: std::io::Lines<BufReader<File>>) -> Result<PuzzleData, String> {
    let eqs = lines
        .map(Result::unwrap)
        .map(|l| {
            let col = l.find(':').expect("input file to be well-formed");
            (
                l[..col].parse().unwrap(),
                l[(col + 1)..]
                    .split_ascii_whitespace()
                    .map(str::parse)
                    .map(Result::unwrap)
                    .collect(),
            )
        })
        .collect();
    Ok(PuzzleData { eqs })
}

fn part1(data: PuzzleData) -> u64 {
    let mut total = 0;
    for eq in data.eqs {
        // println!("{eq:?}");
        let p = 2 << (eq.1.len() - 1) - 1;
        for i in 0..p {
            // print!("\t{} - ", i);
            let mut sum = eq.1[0];
            for j in 0..(eq.1.len() - 1) {
                // print!("{}:{}; ", j, (i >> j) % 2 == 0);
                if (i >> j) % 2 == 0 {
                    sum += eq.1[j + 1];
                } else {
                    sum *= eq.1[j + 1];
                }
            }
            // print!("{sum:?}");
            if sum == eq.0 {
                total += eq.0;
                println!();
                break;
            }
            // println!();
        }
    }
    total
}

fn part2(data: PuzzleData) -> u64 {
    let mut total = 0;
    for eq in data.eqs {
        // println!("{eq:?}");
        let p = 3_usize.pow(eq.1.len() as u32 - 1);
        for i in 0..p {
            // print!("\t{} - ", i);
            let mut sum = eq.1[0];
            let mut r_i = i;
            for j in 0..(eq.1.len() - 1) {
                // print!("{}:{}; ", j, r_i % 3);
                match r_i % 3 {
                    0 => sum += eq.1[j + 1],
                    1 => sum *= eq.1[j + 1],
                    2 => sum = format!("{sum}{}", eq.1[j + 1]).parse().unwrap(),
                    _ => unreachable!(),
                }
                r_i /= 3;
            }
            // print!("{sum:?}");
            if sum == eq.0 {
                total += eq.0;
                // println!();
                break;
            }
            // println!();
        }
    }
    total
}
