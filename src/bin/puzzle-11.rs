// #![feature(linked_list_cursors)]
use std::{
    collections::{HashMap, LinkedList},
    env,
    fs::File as FileFs,
    io::{BufRead, BufReader},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Not enough arguments\nUSAGE: PART ./puzzle-11.exe FILE\n\tWhere PART is one of `1` or `2`");
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
            eprint!("Arguments invalid\nUSAGE: PART ./puzzle-11.exe FILE\n\tWhere PART must be one of `1` or `2`");
            return;
        }
    };
    println!("Got result `{sum}`!");
}

#[derive(Clone, Debug)]
struct PuzzleData {
    stones: Vec<u128>,
}

fn parse_input(lines: std::io::Lines<BufReader<FileFs>>) -> PuzzleData {
    let stones = lines
        .map(Result::unwrap)
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    PuzzleData { stones }
}

fn part1(data: PuzzleData) -> u64 {
    let mut stones = LinkedList::from_iter(data.stones.into_iter());
    // for _i in 0..25 {
    //     // println!("i:= {stones:?}");
    //     let mut c = stones.cursor_front_mut();
    //     while c.index().is_some() {
    //         // print!("{:?}:= ", c.current().unwrap());
    //         let stone = *c.current().unwrap(); // while guard applies
    //         if stone == 0 {
    //             *c.current().unwrap() = 1;
    //             // println!("{:?}", c.current().unwrap());
    //         } else if stone.to_string().len() % 2 == 0 {
    //             let s = stone.to_string();
    //             let left = s[..(s.len() / 2)].parse().unwrap();
    //             let right = s[(s.len() / 2)..].parse().unwrap();
    //             c.insert_before(left);
    //             *c.current().unwrap() = right;
    //             // println!("{:?}", c.current().unwrap());
    //         } else {
    //             *c.current().unwrap() = stone * 2024;
    //             // println!("{:?}", c.current().unwrap());
    //         }
    //         c.move_next();
    //     }
    // }

    stones.len() as u64
}

fn part2(data: PuzzleData) -> u64 {
    let mut stones = data
        .stones
        .into_iter()
        .fold(HashMap::new(), |mut acc, stone| {
            acc.entry(stone).and_modify(|v| *v += 1).or_insert(1);
            acc
        });

    for _i in 0..75 {
        let mut new_stones = HashMap::new();
        for (stone, n) in &stones {
            if *stone == 0 {
                new_stones.entry(1).and_modify(|v| *v += *n).or_insert(*n);
                // println!("{:?}", c.current().unwrap());
            } else if stone.to_string().len() % 2 == 0 {
                let s = stone.to_string();
                let left = s[..(s.len() / 2)].parse().unwrap();
                let right = s[(s.len() / 2)..].parse().unwrap();
                new_stones
                    .entry(left)
                    .and_modify(|v| *v += *n)
                    .or_insert(*n);
                new_stones
                    .entry(right)
                    .and_modify(|v| *v += *n)
                    .or_insert(*n);
                // println!("{:?}", c.current().unwrap());
            } else {
                new_stones
                    .entry(stone * 2024)
                    .and_modify(|v| *v += *n)
                    .or_insert(*n);
                // println!("{:?}", c.current().unwrap());
            }
        }
        stones.clear();
        stones.extend(new_stones);
    }

    stones.into_iter().fold(0, |acc, v| acc + v.1)
}
