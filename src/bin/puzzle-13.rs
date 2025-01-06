use std::{
    env,
    fs::File as FileFs,
    io::{BufRead, BufReader},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Not enough arguments\nUSAGE: PART ./puzzle-13.exe FILE\n\tWhere PART is one of `1` or `2`");
        return;
    }
    let Ok(lines) = FileFs::open(args[2].clone()).map(|f| BufReader::new(f).lines()) else {
        eprintln!("Error reading `{}`", args[2]);
        return;
    };
    let data = parse_input(lines);
    let sum: u64 = match args[1].as_str() {
        "1" => part1(data),
        "2" => part2(data),
        _ => {
            eprint!("Arguments invalid\nUSAGE: PART ./puzzle-13.exe FILE\n\tWhere PART must be one of `1` or `2`");
            return;
        }
    };
    println!("Got result `{sum}`!");
}

#[derive(Clone, Debug, Default)]
struct Machine {
    a: (usize, usize),
    b: (usize, usize),
    prize: (usize, usize),
}
#[derive(Clone, Debug)]
struct PuzzleData {
    machines: Vec<Machine>,
}

fn parse_input(mut lines: std::io::Lines<BufReader<FileFs>>) -> PuzzleData {
    let mut machines = Vec::new();
    let mut curr_m = Machine::default();
    let mut state = 'a';
    while let Some(Ok(l)) = lines.next() {
        match state {
            'a' => {
                let x = l.find('+').unwrap();
                let comma = l.find(',').unwrap();
                let y = l[comma..].find('+').unwrap();
                curr_m.a.0 = l[(x + 1)..comma].parse().unwrap();
                curr_m.a.1 = l[(comma + y + 1)..].trim().parse().unwrap();
                state = 'b';
            }
            'b' => {
                let x = l.find('+').unwrap();
                let comma = l.find(',').unwrap();
                let y = l[comma..].find('+').unwrap();
                curr_m.b.0 = l[(x + 1)..comma].parse().unwrap();
                curr_m.b.1 = l[(comma + y + 1)..].trim().parse().unwrap();
                state = 'p';
            }
            'p' => {
                let x = l.find('=').unwrap();
                let comma = l.find(',').unwrap();
                let y = l[comma..].find('=').unwrap();
                curr_m.prize.0 = l[(x + 1)..comma].parse().unwrap();
                curr_m.prize.1 = l[(comma + y + 1)..].trim().parse().unwrap();
                machines.push(curr_m);
                curr_m = Machine::default();
                state = 'l';
            }
            'l' => state = 'a',
            _ => unreachable!(),
        };
    }
    PuzzleData { machines }
}

fn part1(data: PuzzleData) -> u64 {
    let mut sum = 0;
    for machine in data.machines {
        let mut res = None;
        let det = (machine.a.0 * machine.b.1) as isize - (machine.a.1 * machine.b.0) as isize;
        let na_i =
            (machine.prize.0 * machine.b.1) as isize - (machine.prize.1 * machine.b.0) as isize;
        let nb_i =
            (machine.prize.1 * machine.a.0) as isize - (machine.a.1 * machine.prize.0) as isize;
        if det == 0 && machine.prize.0 % machine.a.0 == 0 && machine.prize.1 % machine.a.1 == 0 {
            if machine.a.0 > 3 * machine.b.0 {
                res = Some((machine.prize.0 / machine.a.0, 0));
            } else {
                res = Some((0, machine.prize.0 / machine.b.0));
            }
        } else if na_i % det == 0 && nb_i % det == 0 {
            let na = na_i / det;
            let nb = nb_i / det;
            if na >= 0 && nb >= 0 {
                res = Some((na as usize, nb as usize));
            }
        }
        let tokens = res.map_or(0, |(a, b)| 3 * a + b);
        println!("{tokens}<={machine:?}");
        sum += tokens;
    }
    sum as u64
}

fn part2(data: PuzzleData) -> u64 {
    let mut sum = 0;
    for mut machine in data.machines {
        machine.prize.0 += 10000000000000;
        machine.prize.1 += 10000000000000;
        let mut res = None;
        let det = (machine.a.0 * machine.b.1) as isize - (machine.a.1 * machine.b.0) as isize;
        let na_i =
            (machine.prize.0 * machine.b.1) as isize - (machine.prize.1 * machine.b.0) as isize;
        let nb_i =
            (machine.prize.1 * machine.a.0) as isize - (machine.a.1 * machine.prize.0) as isize;
        if det == 0 && machine.prize.0 % machine.a.0 == 0 && machine.prize.1 % machine.a.1 == 0 {
            if machine.a.0 > 3 * machine.b.0 {
                res = Some((machine.prize.0 / machine.a.0, 0));
            } else {
                res = Some((0, machine.prize.0 / machine.b.0));
            }
        } else if na_i % det == 0 && nb_i % det == 0 {
            let na = na_i / det;
            let nb = nb_i / det;
            if na >= 0 && nb >= 0 {
                res = Some((na as usize, nb as usize));
            }
        }
        let tokens = res.map_or(0, |(a, b)| 3 * a + b);
        println!("{tokens}<={machine:?}");
        sum += tokens;
    }
    sum as u64
}
