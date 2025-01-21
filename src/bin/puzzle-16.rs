use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    env,
    fs::File as FileFs,
    hash::Hash,
    io::{BufRead, BufReader},
    u64,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Not enough arguments\nUSAGE: PART ./puzzle-16.exe FILE\n\tWhere PART is one of `1` or `2`");
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
            eprint!("Arguments invalid\nUSAGE: PART ./puzzle-16.exe FILE\n\tWhere PART must be one of `1` or `2`");
            return;
        }
    };
    println!("Got result `{sum}`!");
}

#[derive(Clone, Debug)]
struct PuzzleData {
    walls: Vec<(usize, usize)>,
    start: (usize, usize),
    end: (usize, usize),
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}
type Pos = ((usize, usize), Dir);
#[derive(PartialEq, Eq, Clone, Debug)]
struct Path(u64, Pos);

impl Hash for Path {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.1.hash(state);
    }
}

impl From<(u64, ((usize, usize), Dir))> for Path {
    fn from(value: (u64, ((usize, usize), Dir))) -> Self {
        Path(value.0, value.1)
    }
}
impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0).reverse()
    }
}

impl PuzzleData {
    fn next_paths(&self, curr: &Path) -> Vec<Path> {
        match curr.1 .1 {
            Dir::Up => [
                (curr.0 + 1, ((curr.1 .0 .0, curr.1 .0 .1 - 1), Dir::Up)),
                (curr.0 + 1001, ((curr.1 .0 .0 - 1, curr.1 .0 .1), Dir::Left)),
                (
                    curr.0 + 1001,
                    ((curr.1 .0 .0 + 1, curr.1 .0 .1), Dir::Right),
                ),
            ],
            Dir::Right => [
                (curr.0 + 1, ((curr.1 .0 .0 + 1, curr.1 .0 .1), Dir::Right)),
                (curr.0 + 1001, ((curr.1 .0 .0, curr.1 .0 .1 - 1), Dir::Up)),
                (curr.0 + 1001, ((curr.1 .0 .0, curr.1 .0 .1 + 1), Dir::Down)),
            ],
            Dir::Down => [
                (curr.0 + 1, ((curr.1 .0 .0, curr.1 .0 .1 + 1), Dir::Down)),
                (
                    curr.0 + 1001,
                    ((curr.1 .0 .0 + 1, curr.1 .0 .1), Dir::Right),
                ),
                (curr.0 + 1001, ((curr.1 .0 .0 - 1, curr.1 .0 .1), Dir::Left)),
            ],
            Dir::Left => [
                (curr.0 + 1, ((curr.1 .0 .0 - 1, curr.1 .0 .1), Dir::Left)),
                (curr.0 + 1001, ((curr.1 .0 .0, curr.1 .0 .1 - 1), Dir::Up)),
                (curr.0 + 1001, ((curr.1 .0 .0, curr.1 .0 .1 + 1), Dir::Down)),
            ],
        }
        .into_iter()
        .filter(|p| self.walls.binary_search(&p.1 .0).is_err())
        .map(|v| v.into())
        .collect()
    }

    // fn heuristic(&self, path: &Path) -> u64 {
    //     let dist = (
    //         self.end.0.abs_diff(path.1 .0 .0),
    //         self.end.1.abs_diff(path.1 .0 .1),
    //     );
    //     let signs = (self.end.0 < path.1 .0 .0, self.end.1 < path.1 .0 .1);
    //     let turns = match dist {
    //         (0, 0) => 0,
    //         (_, 0) => {
    //             if matches!(path.1 .1, Dir::Left) {
    //                 if signs.0 {
    //                     0
    //                 } else {
    //                     2
    //                 }
    //             } else if matches!(path.1 .1, Dir::Right) {
    //                 if !signs.0 {
    //                     0
    //                 } else {
    //                     2
    //                 }
    //             } else {
    //                 1
    //             }
    //         }
    //         (0, _) => {
    //             if matches!(path.1 .1, Dir::Up) {
    //                 if signs.1 {
    //                     0
    //                 } else {
    //                     2
    //                 }
    //             } else if matches!(path.1 .1, Dir::Down) {
    //                 if !signs.1 {
    //                     0
    //                 } else {
    //                     2
    //                 }
    //             } else {
    //                 1
    //             }
    //         }
    //         _ => match path.1 .1 {
    //             Dir::Up => {
    //                 if signs.0 == signs.1 {
    //                     1
    //                 } else {
    //                     2
    //                 }
    //             }
    //             Dir::Right => {
    //                 if !signs.0 {
    //                     1
    //                 } else {
    //                     2
    //                 }
    //             }
    //             Dir::Down => {
    //                 if signs.0 != signs.1 {
    //                     1
    //                 } else {
    //                     2
    //                 }
    //             }
    //             Dir::Left => {
    //                 if signs.0 {
    //                     1
    //                 } else {
    //                     2
    //                 }
    //             }
    //         },
    //     };
    //     (dist.0 + dist.1 + 1000 * turns) as u64
    // }
}

impl Dir {
    fn turns_from(&self, other: &Dir) -> u64 {
        match (self, other) {
            (Dir::Up, Dir::Up)
            | (Dir::Right, Dir::Right)
            | (Dir::Down, Dir::Down)
            | (Dir::Left, Dir::Left) => 0,
            (Dir::Up, Dir::Right) | (Dir::Right, Dir::Up) => 1,
            (Dir::Right, Dir::Down) | (Dir::Down, Dir::Right) => 1,
            (Dir::Down, Dir::Left) | (Dir::Left, Dir::Down) => 1,
            (Dir::Up, Dir::Left) | (Dir::Left, Dir::Up) => 1,
            (Dir::Right, Dir::Left)
            | (Dir::Left, Dir::Right)
            | (Dir::Up, Dir::Down)
            | (Dir::Down, Dir::Up) => 0,
        }
    }

    // fn true_turns_from(&self, other: &Dir) -> u64 {
    //     match (self, other) {
    //         (Dir::Up, Dir::Up)
    //         | (Dir::Right, Dir::Right)
    //         | (Dir::Down, Dir::Down)
    //         | (Dir::Left, Dir::Left) => 0,
    //         (Dir::Up, Dir::Right) | (Dir::Right, Dir::Up) => 1,
    //         (Dir::Right, Dir::Down) | (Dir::Down, Dir::Right) => 1,
    //         (Dir::Down, Dir::Left) | (Dir::Left, Dir::Down) => 1,
    //         (Dir::Up, Dir::Left) | (Dir::Left, Dir::Up) => 1,
    //         (Dir::Right, Dir::Left)
    //         | (Dir::Left, Dir::Right)
    //         | (Dir::Up, Dir::Down)
    //         | (Dir::Down, Dir::Up) => 2,
    //     }
    // }
}

fn parse_input(lines: std::io::Lines<BufReader<FileFs>>) -> PuzzleData {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut walls: Vec<(usize, usize)> = lines
        .map(Result::unwrap)
        .enumerate()
        .flat_map(|(i, l)| {
            l.char_indices()
                .filter_map(|(j, c)| match c {
                    '#' => Some((j, i)),
                    'E' => {
                        end = (j, i);
                        None
                    }
                    'S' => {
                        start = (j, i);
                        None
                    }
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .collect();
    walls.sort();
    PuzzleData { walls, start, end }
}

fn part1(data: PuzzleData) -> u64 {
    let path = Path(0, (data.start, Dir::Right));
    let mut paths: BinaryHeap<Path> = BinaryHeap::new();
    let mut min = u64::MAX;
    let mut visited: Vec<Path> = Vec::new();
    paths.extend(data.next_paths(&path));
    while let Some(p) = paths.pop() {
        if let Some(v) = visited.iter_mut().find(|v| v.1 == p.1) {
            if p.0 < v.0 {
                *v = p.clone();
            } else {
                continue;
            }
        }
        if p.1 .0 == data.end {
            if p.0 < min {
                min = p.0;
            }
        }
        paths.extend(data.next_paths(&p));
        visited.push(p);
    }

    min
}

// 492 CHEATED
fn part2(data: PuzzleData) -> u64 {
    // Compute predecessors
    let mut predecessors: HashMap<(usize, usize), Vec<Path>> = HashMap::new();
    let mut paths_to_visit = BinaryHeap::from([Path(0, (data.start, Dir::Right))]);
    let mut visited = HashSet::new();
    while let Some(path) = paths_to_visit.pop() {
        let neighbours = data.next_paths(&path);
        for new_path in neighbours {
            if visited.contains(&new_path.1 .0) {
                predecessors
                    .entry(new_path.1 .0)
                    .and_modify(|e| e.push(path.clone()));
            } else if let Some(v) = paths_to_visit.iter().find(|e| e.1 .0 == new_path.1 .0) {
            } else {
                predecessors.insert(new_path.1 .0, vec![path.clone()]);
                paths_to_visit.push(new_path);
            }
        }
        visited.insert(path.1 .0);
    }

    // Reconstruct paths
    let mut path = HashSet::new();
    path.insert(data.end);
    // let new = predecessors.remove(&data.end).unwrap();
    // let min = new.iter().min_by_key(|p| p.0).unwrap().0;

    let new = predecessors.remove(&data.end).unwrap();
    let new = new.into_iter().map(|mut e| {
        if e.1 .0 .0 < data.end.0 {
            // Right
            e.0 += 1000 * e.1 .1.turns_from(&Dir::Right);
            e.1 .1 = Dir::Right;
        } else if e.1 .0 .0 > data.end.0 {
            // Left
            e.0 += 1000 * e.1 .1.turns_from(&Dir::Left);
            e.1 .1 = Dir::Left;
        } else if e.1 .0 .1 < data.end.1 {
            // Down
            e.0 += 1000 * e.1 .1.turns_from(&Dir::Down);
            e.1 .1 = Dir::Down;
        } else if e.1 .0 .1 > data.end.1 {
            // Up
            e.0 += 1000 * e.1 .1.turns_from(&Dir::Up);
            e.1 .1 = Dir::Up;
        }
        e
    });
    let mut curr_preds: Vec<_> = new.collect();
    while let Some(p) = curr_preds.pop() {
        if path.contains(&p.1 .0) {
            continue;
        }
        if p.1 .0 == data.start {
            path.insert(p.1 .0);
            continue;
        }
        let new = predecessors.remove(&p.1 .0).unwrap();
        let new = new
            .into_iter()
            .filter(|e| {
                e.1 .1 == p.1 .1 && e.0 == p.0 - 1
                    || e.0 + 1 + 1000 * e.1 .1.turns_from(&p.1 .1) == p.0
            })
            .map(|mut e| {
                if e.1 .0 .0 < p.1 .0 .0 {
                    // Right
                    e.0 += 1000 * e.1 .1.turns_from(&Dir::Right);
                    e.1 .1 = Dir::Right;
                } else if e.1 .0 .0 > p.1 .0 .0 {
                    // Left
                    e.0 += 1000 * e.1 .1.turns_from(&Dir::Left);
                    e.1 .1 = Dir::Left;
                } else if e.1 .0 .1 < p.1 .0 .1 {
                    // Down
                    e.0 += 1000 * e.1 .1.turns_from(&Dir::Down);
                    e.1 .1 = Dir::Down;
                } else if e.1 .0 .1 > p.1 .0 .1 {
                    // Up
                    e.0 += 1000 * e.1 .1.turns_from(&Dir::Up);
                    e.1 .1 = Dir::Up;
                }
                e
            });
        println!("{p:?} <= {new:?}");
        curr_preds.extend(new);

        path.insert(p.1 .0);
    }
    const SIZE: usize = 141;
    let mut canvas: Vec<Vec<u8>> = vec![vec![b' '; SIZE]; SIZE];
    for p in &path {
        canvas[p.1][p.0] = b'O';
    }
    for w in data.walls {
        canvas[w.1][w.0] = b'#';
    }
    for l in canvas {
        println!("{}", String::from_utf8(l).unwrap());
    }
    path.len() as u64
}
