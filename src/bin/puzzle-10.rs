use std::{
    collections::{HashMap, HashSet},
    convert::identity,
    env,
    fs::File as FileFs,
    io::{self, BufRead, BufReader},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Not enough arguments\nUSAGE: PART ./puzzle-10.exe FILE\n\tWhere PART is one of `1` or `2`");
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
            eprint!("Arguments invalid\nUSAGE: PART ./puzzle-10.exe FILE\n\tWhere PART must be one of `1` or `2`");
            return;
        }
    };
    println!("Got result `{sum}`!");
}

#[derive(Clone, Debug)]
struct PuzzleData {
    heights: Vec<u8>,
    width: usize,
}

impl PuzzleData {
    fn height(&self, coord: (usize, usize)) -> Option<u8> {
        let index = self.from_coord(coord);
        self.heights.get(index).copied()
    }

    // Order is Top Right Down Left
    fn surrounding(&self, coord: (usize, usize)) -> [Option<(u8, (usize, usize))>; 4] {
        [
            coord.1.gt(&0).then(|| {
                (
                    self.height((coord.0, coord.1 - 1)).unwrap(),
                    (coord.0, coord.1 - 1),
                )
            }),
            coord.0.lt(&(self.width - 1)).then(|| {
                (
                    self.height((coord.0 + 1, coord.1)).unwrap(),
                    (coord.0 + 1, coord.1),
                )
            }),
            coord.1.lt(&(self.width - 1)).then(|| {
                (
                    self.height((coord.0, coord.1 + 1)).unwrap(),
                    (coord.0, coord.1 + 1),
                )
            }),
            coord.0.gt(&0).then(|| {
                (
                    self.height((coord.0 - 1, coord.1)).unwrap(),
                    (coord.0 - 1, coord.1),
                )
            }),
        ]
    }

    fn from_index(&self, index: usize) -> (usize, usize) {
        (index % self.width, index / self.width)
    }

    fn from_coord(&self, coord: (usize, usize)) -> usize {
        coord.0 + self.width * coord.1
    }
}

fn parse_input(lines: std::io::Lines<BufReader<FileFs>>) -> PuzzleData {
    let heights: Vec<u8> = lines
        .map(Result::unwrap)
        .flat_map(|l| {
            l.chars()
                .map(|c| c.to_digit(16).unwrap() as u8)
                .collect::<Vec<_>>()
        })
        .collect();
    unsafe {
        PuzzleData {
            width: (heights.len() as f64).sqrt().to_int_unchecked(),
            heights,
        }
    }
}

fn part1(data: PuzzleData) -> u64 {
    let mut scores = 0;
    let starts = data
        .heights
        .iter()
        .enumerate()
        .filter_map(|(i, h)| h.eq(&0).then_some(i));
    for start in starts {
        let mut visited = HashSet::new();
        visited.insert(start);
        let mut last_len = 0;
        while visited.len() != last_len {
            last_len = visited.len();
            let mut new_v = Vec::new();
            for v in &visited {
                for c in data
                    .surrounding(data.from_index(*v))
                    .into_iter()
                    .filter_map(identity)
                    .filter_map(|n| (n.0 == data.heights[*v] + 1).then_some(n.1))
                {
                    new_v.push(data.from_coord(c));
                }
            }
            visited.extend(new_v.into_iter());
        }

        scores += visited.iter().filter(|v| data.heights[**v] == 9).count();
    }
    scores as u64
}

fn part2(data: PuzzleData) -> u64 {
    let mut scores = 0;
    let starts = data
        .heights
        .iter()
        .enumerate()
        .filter_map(|(i, h)| h.eq(&0).then_some(i));
    for start in starts {
        let mut last_visited = HashMap::new();
        last_visited.insert(start, 1u64);
        for i in 1..10 {
            let mut visited = HashMap::new();
            for (index, visits) in last_visited.iter() {
                data.surrounding(data.from_index(*index))
                    .into_iter()
                    .filter_map(identity)
                    .filter_map(|n| (n.0 == i).then_some(n.1))
                    .for_each(|c| {
                        visited
                            .entry(data.from_coord(c))
                            .and_modify(|v| *v += *visits)
                            .or_insert(*visits);
                    });
            }
            last_visited.clear();
            last_visited.extend(visited.into_iter());
        }

        scores += last_visited.iter().fold(0, |acc, v| acc + v.1);
    }
    scores as u64
}
