#![allow(dead_code)]

use std::{
    collections::{HashMap, HashSet},
    env,
    fs::File as FileFs,
    io::Read,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Not enough arguments\nUSAGE: PART ./puzzle-20.exe FILE\n\tWhere PART is one of `1` or `2`");
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
            eprint!("Arguments invalid\nUSAGE: PART ./puzzle-20.exe FILE\n\tWhere PART must be one of `1` or `2`");
        }
    }
}

type PuzzleResult = usize;
type Coord = (usize, usize);
#[derive(Clone, Debug)]
struct PuzzleData {
    width: usize,
    walls: HashSet<Coord>,
    start: Coord,
    end: Coord,
}

fn parse_input(data: &str) -> PuzzleData {
    let mut width = 0;
    let mut start = Coord::default();
    let mut end = Coord::default();
    let walls = data
        .lines()
        .into_iter()
        .enumerate()
        .flat_map(|(y, l)| {
            width = l.len();
            l.chars()
                .enumerate()
                .filter_map(|(x, c)| match c {
                    '#' => Some((x, y)),
                    'S' => {
                        start = (x, y);
                        None
                    }
                    'E' => {
                        end = (x, y);
                        None
                    }
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .collect();

    PuzzleData {
        width,
        walls,
        start,
        end,
    }
}

impl PuzzleData {
    fn compute_path(&self) -> Vec<Coord> {
        let mut res = Vec::new();
        let mut curr = self.start;
        while curr != self.end {
            res.push(curr);
            if !self.walls.contains(&(curr.0 - 1, curr.1)) && !res.contains(&(curr.0 - 1, curr.1)) {
                curr = (curr.0 - 1, curr.1);
            } else if !self.walls.contains(&(curr.0 + 1, curr.1))
                && !res.contains(&(curr.0 + 1, curr.1))
            {
                curr = (curr.0 + 1, curr.1);
            } else if !self.walls.contains(&(curr.0, curr.1 - 1))
                && !res.contains(&(curr.0, curr.1 - 1))
            {
                curr = (curr.0, curr.1 - 1);
            } else if !self.walls.contains(&(curr.0, curr.1 + 1))
                && !res.contains(&(curr.0, curr.1 + 1))
            {
                curr = (curr.0, curr.1 + 1);
            }
        }
        res.push(self.end);
        res
    }

    fn compute_path_map(&self) -> HashMap<Coord, usize> {
        self.compute_path()
            .into_iter()
            .enumerate()
            .map(|(a, b)| (b, a))
            .collect()
    }
    fn cheats(&self, cheat_time: usize) -> Vec<(usize, Coord, Coord)> {
        if cheat_time < 2 {
            return vec![];
        }
        let mut cheats = Vec::new();
        let path_map = self.compute_path_map();
        let path = self.compute_path();
        for (i, p) in path.iter().enumerate() {
            for other in &path[(i + 1)..] {
                if dist(p, other) <= cheat_time && dist(p, other) > 1 {
                    let time_saved = path_map[other] - path_map[p] - dist(p, other);
                    if time_saved > 0 {
                        cheats.push((time_saved, p.clone(), other.clone()));
                    }
                }
            }
        }
        cheats
    }
}

fn dist(c1: &Coord, c2: &Coord) -> usize {
    c1.0.abs_diff(c2.0) + c1.1.abs_diff(c2.1)
}
fn part1(data: PuzzleData) -> PuzzleResult {
    let res = data.cheats(2).iter().filter(|c| c.0 >= 100).count();
    println!("{res}");
    res
}
fn part2(data: PuzzleData) -> PuzzleResult {
    let res = data.cheats(20).iter().filter(|c| c.0 >= 100).count();
    println!("{res}");
    res
}

#[cfg(test)]
mod tests {

    use crate::*;
    mod examples {
        pub const EX_1: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
    }

    #[test]
    fn test_path() {
        let data = parse_input(examples::EX_1);

        assert_eq!(data.compute_path().len(), 85);
    }
    #[test]
    fn test_cheats() {
        let data = parse_input(examples::EX_1);
        let cheats = data.cheats(2);
        assert_eq!(cheats.len(), 44);
        assert!(cheats.iter().find(|c| c.0 == 64).is_some());
        assert!(cheats.iter().find(|c| c.0 > 64).is_none());
        assert!(cheats.iter().find(|c| c.0 < 2).is_none());
        let cheats: Vec<_> = data.cheats(20).into_iter().filter(|c| c.0 >= 50).collect();
        assert_eq!(cheats.len(), 285);
        assert!(cheats.iter().find(|c| c.0 == 76).is_some());
        assert!(cheats.iter().find(|c| c.0 > 76).is_none());
    }

    #[test]
    fn test_2() {}
}
