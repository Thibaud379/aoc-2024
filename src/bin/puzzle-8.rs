use std::{
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::{BufRead, BufReader},
    iter,
    ops::Sub,
};

use gcd::Gcd;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Not enough arguments\nUSAGE: PART ./puzzle-8.exe FILE\n\tWhere PART is one of `1` or `2`");
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
            eprint!("Arguments invalid\nUSAGE: PART ./puzzle-8.exe FILE\n\tWhere PART must be one of `1` or `2`");
            return;
        }
    };
    println!("Got result `{sum}`!");
}

#[derive(Clone, Debug)]
struct PuzzleData {
    antennas: HashMap<char, Vec<(usize, usize)>>,
    width: usize,
}

fn parse_input(lines: std::io::Lines<BufReader<File>>) -> Result<PuzzleData, String> {
    let mut antennas = HashMap::new();
    let mut width_o = None;
    lines.map(Result::unwrap).enumerate().for_each(|(y, l)| {
        width_o.get_or_insert(l.len());
        for (x, c) in l.chars().enumerate().filter(|v| v.1.ne(&'.')) {
            antennas
                .entry(c)
                .and_modify(|v: &mut Vec<(usize, usize)>| v.push((x, y)))
                .or_insert(vec![(x, y)]);
        }
        if width_o.is_some_and(|v| v <= y) {
            eprintln!("oupsi a rect");
        }
    });
    Ok(PuzzleData {
        antennas,
        width: width_o.unwrap(),
    })
}

fn add(a: &(usize, usize), b: &(isize, isize)) -> Option<(usize, usize)> {
    let res = (a.0.checked_add_signed(b.0), a.1.checked_add_signed(b.1));
    res.0.and(res.1)?;
    Some((res.0.unwrap(), res.1.unwrap()))
}

fn sub(a: &(usize, usize), b: &(isize, isize)) -> Option<(usize, usize)> {
    let res = (a.0.checked_add_signed(-b.0), a.1.checked_add_signed(-b.1));
    res.0.and(res.1)?;
    Some((res.0.unwrap(), res.1.unwrap()))
}

fn diff(a: &(usize, usize), b: &(usize, usize)) -> (isize, isize) {
    let res = (
        (a.0 as isize).checked_sub_unsigned(b.0),
        (a.1 as isize).checked_sub_unsigned(b.1),
    );
    (res.0.unwrap(), res.1.unwrap())
}

fn fits(a: &(usize, usize), size: usize) -> bool {
    a.0 < size && a.1 < size
}

fn fits_o(a: &Option<(usize, usize)>, size: usize) -> bool {
    a.is_some_and(|a| a.0 < size && a.1 < size)
}

fn part1(data: PuzzleData) -> u64 {
    let antenna_locs = data.antennas.values().fold(HashSet::new(), |mut acc, v| {
        acc.extend(v.iter().cloned());
        acc
    });
    let mut antinodes = HashSet::new();
    for ants in data.antennas.values() {
        for pair in ants
            .iter()
            .flat_map(|l| ants.iter().map(move |r| (l, r)))
            .filter(|(x, y)| *x != *y)
        {
            let diff = diff(pair.0, pair.1);
            let antis = vec![sub(pair.1, &diff), add(pair.0, &diff)];
            antinodes.extend(
                antis
                    .into_iter()
                    .filter(|o| o.is_some_and(|a| fits(&a, data.width)))
                    .map(Option::unwrap),
            );
        }
    }

    antinodes.len() as u64
}

fn part2(data: PuzzleData) -> u64 {
    let antenna_locs = data.antennas.values().fold(HashSet::new(), |mut acc, v| {
        acc.extend(v.iter().cloned());
        acc
    });
    let mut antinodes = HashSet::new();
    for ants in data.antennas.values() {
        for pair in ants
            .iter()
            .flat_map(|l| ants.iter().map(move |r| (l, r)))
            .filter(|(x, y)| *x != *y)
        {
            let diff = diff(pair.0, pair.1);
            let diff_gcd = (diff.0.abs() as usize).gcd(diff.1.abs() as usize) as isize;
            print!("{:?} - ", diff);
            print!("{:?} - ", diff_gcd);
            let diff = (diff.0 / diff_gcd, diff.1 / diff_gcd);
            println!("{:?}", diff);
            let mut antis = Vec::new();
            let mut p = Some(pair.0.clone());
            while fits_o(&p, data.width) {
                antis.push(p.unwrap());
                p = add(&p.unwrap(), &diff);
            }
            let mut p = Some(pair.1.clone());
            while fits_o(&p, data.width) {
                antis.push(p.unwrap());
                p = sub(&p.unwrap(), &diff);
            }
            antinodes.extend(antis);
        }
    }
    antinodes.len() as u64
}
