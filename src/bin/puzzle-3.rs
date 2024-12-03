use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Not enough arguments\nUSAGE: PART ./puzzle-3.exe FILE\n\tWhere PART is one of `1` or `2`");
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
            eprint!("Arguments invalid\nUSAGE: PART ./puzzle-3.exe FILE\n\tWhere PART must be one of `1` or `2`");
            return;
        }
    };
    println!("Got result `{sum}`!");
}

fn part1(lines: std::io::Lines<BufReader<File>>) -> u64 {
    lines
        .map(Result::unwrap)
        .map(|l| {
            let mut s = l.as_str();
            let mut sum = 0;
            while let Some(idx) = s.find("mul(") {
                s = &s[(idx + 4)..];
                let Some(comma) = s.find(',') else {
                    break;
                };
                if comma == 0 || comma > 3 || s[..comma].contains(char::is_whitespace) {
                    continue;
                }
                let Ok(left) = s[..comma].parse::<u64>() else {
                    s = &s[comma..];
                    continue;
                };
                s = &s[(comma + 1)..];
                let Some(par) = s.find(')') else {
                    break;
                };

                if par == 0 || par > 3 || s[..par].contains(char::is_whitespace) {
                    continue;
                }
                let Ok(right) = s[..par].parse::<u64>() else {
                    s = &s[par..];
                    continue;
                };
                println!("mul({left}*{right})");
                s = &s[(par + 1)..];
                sum += right * left;
            }
            sum
        })
        .sum()
}

fn part2(lines: std::io::Lines<BufReader<File>>) -> u64 {
    let mut enabled = true;
    lines
        .map(Result::unwrap)
        .map(|l| {
            let mut s = l.as_str();
            let mut sum = 0;
            while let Some(idx) = s.find("mul(") {
                let enables = (s.find("do()"), s.find("don't()"));
                match enables {
                    (Some(d), Some(dont)) if dont < idx && d < idx => enabled = d > dont,
                    (_, Some(dont)) if dont < idx => enabled = false,
                    (Some(d), _) if d < idx => enabled = true,
                    _ => (),
                }

                println!("{enables:?} - {enabled}");
                s = &s[(idx + 4)..];
                if !enabled {
                    continue;
                }
                let Some(comma) = s.find(',') else {
                    break;
                };
                if comma == 0 || comma > 3 || s[..comma].contains(char::is_whitespace) {
                    continue;
                }
                let Ok(left) = s[..comma].parse::<u64>() else {
                    s = &s[comma..];
                    continue;
                };
                s = &s[(comma + 1)..];
                let Some(par) = s.find(')') else {
                    break;
                };

                if par == 0 || par > 3 || s[..par].contains(char::is_whitespace) {
                    continue;
                }
                let Ok(right) = s[..par].parse::<u64>() else {
                    s = &s[par..];
                    continue;
                };
                println!("mul({left}*{right})");
                s = &s[(par + 1)..];
                sum += right * left;
            }
            sum
        })
        .sum()
}
