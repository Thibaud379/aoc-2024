#![allow(dead_code)]
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    env,
    fs::File as FileFs,
    io::Read,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Not enough arguments\nUSAGE: PART ./puzzle-18.exe FILE\n\tWhere PART is one of `1` or `2`");
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
            part1(&data);
        }
        "2" => {
            part2(data);
        }
        _ => {
            eprint!("Arguments invalid\nUSAGE: PART ./puzzle-18.exe FILE\n\tWhere PART must be one of `1` or `2`");
        }
    }
}

type PuzzleResult = Option<usize>;

#[derive(Clone, Debug)]
struct PuzzleData {
    width: usize,
    fallen: usize,
    bytes: Vec<(usize, usize)>,
}

fn parse_input(data: &str) -> PuzzleData {
    let bytes = data
        .lines()
        .map(|l| {
            let comma = l.find(',').unwrap();
            (
                l[..comma].parse().unwrap(),
                l[(comma + 1)..].parse().unwrap(),
            )
        })
        .collect();
    PuzzleData {
        width: 71,
        bytes,
        fallen: 1024,
    }
}

impl PuzzleData {
    fn fallen_bytes(&self) -> &[(usize, usize)] {
        &self.bytes.as_slice()[..self.fallen]
    }

    fn neigbhours(&self, node: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        [
            (node.0 > 0).then(|| (node.0 - 1, node.1)),
            (node.1 > 0).then(|| (node.0, node.1 - 1)),
            (node.0 < self.width - 1).then(|| (node.0 + 1, node.1)),
            (node.1 < self.width - 1).then(|| (node.0, node.1 + 1)),
        ]
        .into_iter()
        .flatten()
    }
}

fn part1(data: &PuzzleData) -> (HashSet<(usize, usize)>, Option<usize>) {
    let start = (0usize, 0usize);
    let end = (data.width - 1, data.width - 1);

    let mut distance = vec![vec![None; data.width]; data.width];
    distance[0][0] = Some(0);
    let mut visited = HashSet::new();
    visited.insert(start);
    let mut to_visit = BinaryHeap::new();
    to_visit.push((Reverse(0), start));
    let mut predecessors = HashMap::new();
    while let Some(next) = to_visit.pop() {
        for n in data.neigbhours(next.1) {
            if !data.fallen_bytes().contains(&n) && distance[n.0][n.1]
                    .is_none_or(|v| v > distance[next.1 .0][next.1 .1].unwrap() + 1) {
                distance[n.0][n.1] = Some(distance[next.1 .0][next.1 .1].unwrap() + 1);
                to_visit.push((Reverse(distance[next.1 .0][next.1 .1].unwrap() + 1), n));
                predecessors.insert(n, next.1);
            }
        }
        visited.insert(next.1);
    }
    const SIZE: usize = 71;
    let mut canvas: Vec<Vec<u8>> = vec![vec![b' '; SIZE]; SIZE];
    for p in data.fallen_bytes() {
        canvas[p.1][p.0] = b'#';
    }
    canvas[end.1][end.0] = b'E';
    canvas[0][0] = b'S';
    let mut pos = end;
    let mut path = HashSet::new();
    while let Some(p) = predecessors.get(&pos) {
        canvas[p.1][p.0] = b'O';
        pos = *p;
        path.insert(*p);
    }

    for l in canvas {
        println!("{}", String::from_utf8(l).unwrap());
    }
    println!("{:?}", distance[end.0][end.1]);
    (path, distance[end.0][end.1])
}
fn part2(mut data: PuzzleData) -> (usize, usize) {
    while let (p, Some(_)) = part1(&data) {
        data.fallen += 1;
        while !p.contains(&data.bytes[data.fallen - 1]) {
            data.fallen += 1;
        }
    }
    println!("{:?}", &data.bytes[data.fallen - 1]);
    data.bytes[data.fallen - 1]
}

#[cfg(test)]
mod tests {
    use crate::*;
    mod examples {
        pub const EX_1: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
    }

    #[test]
    fn test_1() {
        let mut data = parse_input(examples::EX_1);
        data.width = 7;
        data.fallen = 12;
        assert!(matches!(part1(&data), (_, Some(22))));
    }
    #[test]
    fn test_2() {
        let mut data = parse_input(examples::EX_1);
        data.width = 7;
        data.fallen = 12;
        assert_eq!(part2(data), (6, 1));
    }
}
