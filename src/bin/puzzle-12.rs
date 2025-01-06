use std::{
    collections::{HashMap, HashSet},
    env,
    fs::File as FileFs,
    io::{BufRead, BufReader},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Not enough arguments\nUSAGE: PART ./puzzle-12.exe FILE\n\tWhere PART is one of `1` or `2`");
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
            eprint!("Arguments invalid\nUSAGE: PART ./puzzle-12.exe FILE\n\tWhere PART must be one of `1` or `2`");
            return;
        }
    };
    println!("Got result `{sum}`!");
}

#[derive(Clone, Debug)]
struct PuzzleData {
    plots: Vec<char>,
    width: usize,
}

fn parse_input(lines: std::io::Lines<BufReader<FileFs>>) -> PuzzleData {
    let plots: Vec<char> = lines
        .map(Result::unwrap)
        .flat_map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    unsafe {
        PuzzleData {
            width: (plots.len() as f64).sqrt().to_int_unchecked(),
            plots,
        }
    }
}

impl PuzzleData {
    // Order is Top Down Left Right
    fn surrounding(&self, index: usize) -> Vec<(usize, char)> {
        [
            index
                .ge(&self.width)
                .then(|| (index - self.width, self.plots[index - self.width])),
            index
                .lt(&(self.width * (self.width - 1)))
                .then(|| (index + self.width, self.plots[index + self.width])),
            (index % self.width)
                .gt(&0)
                .then(|| (index - 1, self.plots[index - 1])),
            (index % self.width)
                .lt(&(self.width - 1))
                .then(|| (index + 1, self.plots[index + 1])),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    fn index_to_coord(&self, index: usize) -> (usize, usize) {
        (index % self.width, index / self.width)
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
struct Edge {
    vertical: bool,
    orientation: bool,
    start: (usize, usize),
    len: usize,
}

impl Edge {
    fn new(coord1: (usize, usize), coord2: (usize, usize), orientation: bool) -> Option<Self> {
        let difx = coord1.0.abs_diff(coord2.0);
        let dify = coord1.1.abs_diff(coord2.1);
        if difx != 0 && dify != 0 || difx == dify {
            return None;
        }
        if difx == 0 {
            Some(Edge {
                len: dify,
                vertical: true,
                start: if coord1.1 > coord2.1 { coord2 } else { coord1 },
                orientation,
            })
        } else {
            Some(Edge {
                len: difx,
                vertical: false,
                start: if coord1.0 > coord2.0 { coord2 } else { coord1 },
                orientation,
            })
        }
    }

    fn contains(&self, other: &Self) -> bool {
        self.vertical == other.vertical
            && ((self.vertical
                && self.start.0 == other.start.0
                && other.start.1 >= self.start.1
                && (other.start.1 + other.len) <= (self.start.1 + self.len))
                || (!self.vertical
                    && self.start.1 == other.start.1
                    && other.start.0 >= self.start.0
                    && (other.start.0 + other.len) <= (self.start.0 + self.len)))
    }
    fn extends(&self, other: &Self) -> bool {
        self.vertical == other.vertical
            && self.orientation == other.orientation
            && ((self.vertical
                && self.start.0 == other.start.0
                && (other.start.1 == (self.start.1 + self.len)
                    || (other.start.1 + other.len) == self.start.1))
                || (!self.vertical
                    && self.start.1 == other.start.1
                    && (other.start.0 == (self.start.0 + self.len)
                        || (other.start.0 + other.len) == self.start.0)))
    }

    fn sub(&self, other: &Self) -> Vec<Self> {
        if !self.contains(other) {
            vec![Some(self.clone())]
        } else {
            if self.vertical {
                if self.start.1 == other.start.1 {
                    vec![Edge::new(
                        (self.start.0, self.start.1 + other.len),
                        (self.start.0, self.start.1 + self.len),
                        self.orientation,
                    )]
                } else if self.start.1 + self.len == other.start.1 + other.len {
                    vec![Edge::new(
                        (self.start.0, self.start.1),
                        (self.start.0, self.start.1 + self.len - other.len),
                        self.orientation,
                    )]
                } else {
                    vec![
                        Edge::new(
                            (self.start.0, self.start.1),
                            (self.start.0, other.start.1),
                            self.orientation,
                        ),
                        Edge::new(
                            (self.start.0, other.start.1 + other.len),
                            (self.start.0, self.start.1 + self.len),
                            self.orientation,
                        ),
                    ]
                }
            } else {
                if self.start.0 == other.start.0 {
                    vec![Edge::new(
                        (self.start.0 + other.len, self.start.1),
                        (self.start.0 + self.len, self.start.1),
                        self.orientation,
                    )]
                } else if self.start.0 + self.len == other.start.0 + other.len {
                    vec![Edge::new(
                        (self.start.0, self.start.1),
                        (self.start.0 + self.len - other.len, self.start.1),
                        self.orientation,
                    )]
                } else {
                    vec![
                        Edge::new(
                            (self.start.0, self.start.1),
                            (other.start.0, self.start.1),
                            self.orientation,
                        ),
                        Edge::new(
                            (other.start.0 + other.len, self.start.1),
                            (self.start.0 + self.len, self.start.1),
                            self.orientation,
                        ),
                    ]
                }
            }
        }
        .into_iter()
        .flatten()
        .collect()
    }

    fn extend(&mut self, other: &Self) -> bool {
        if !self.extends(other) {
            return false;
        } else {
            if self.vertical {
                if self.start.1 == other.start.1 + other.len {
                    self.start = other.start;
                    self.len += other.len;
                } else if self.start.1 + self.len == other.start.1 {
                    self.len += other.len;
                } else {
                    unreachable!()
                }
            } else {
                if self.start.0 == other.start.0 + other.len {
                    self.start = other.start;
                    self.len += other.len;
                } else if self.start.0 + self.len == other.start.0 {
                    self.len += other.len;
                } else {
                    unreachable!()
                }
            };
        }
        true
    }

    fn plot_edges(coord: (usize, usize)) -> Vec<Edge> {
        vec![
            Edge::new(coord, (coord.0 + 1, coord.1), true),  // top
            Edge::new(coord, (coord.0, coord.1 + 1), false), // left
            Edge::new((coord.0 + 1, coord.1), (coord.0 + 1, coord.1 + 1), true), //right
            Edge::new((coord.0, coord.1 + 1), (coord.0 + 1, coord.1 + 1), false), //bottom
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

fn part1(data: PuzzleData) -> u64 {
    let mut assigned = HashSet::new();
    let mut sum = 0;
    while assigned.len() != data.plots.len() {
        let (start, start_tag) = data
            .plots
            .iter()
            .enumerate()
            .find_map(|(i, p)| (!assigned.contains(&i)).then_some((i, *p)))
            .unwrap();
        let mut region = HashSet::from([start]);
        let mut visited = HashSet::from([start]);
        let mut last_len = 0;
        let mut area = 1;
        let mut perimeter: u64 = 4;
        while last_len != region.len() {
            last_len = region.len();
            let mut new_visited = HashMap::new();
            for p in &visited {
                data.surrounding(*p)
                    .into_iter()
                    .filter_map(|(i, c)| c.eq(&start_tag).then_some(i))
                    .for_each(|i| {
                        new_visited.entry(i).and_modify(|v| *v += 1).or_insert(1);
                    });
            }
            // println!("{new_visited:?}");
            for n in new_visited
                .iter()
                .filter_map(|v| (!region.contains(v.0)).then_some(v.1))
            {
                // println!("{area} - {perimeter}");
                area += 1;
                perimeter = perimeter
                    .checked_add_signed(match n {
                        1 => 2,
                        2 => 0,
                        3 => -2,
                        4 => -4,
                        _ => unreachable!(),
                    })
                    .unwrap();
            }

            visited.clear();
            region.extend(new_visited.keys());
            visited.extend(new_visited.keys());
        }
        println!(
            "Region {}: {}*{}={} {:?}",
            start_tag,
            area,
            perimeter,
            area * perimeter,
            region
        );
        sum += area * perimeter;
        assigned.extend(region);
    }
    sum
}

fn part2(data: PuzzleData) -> u64 {
    let mut assigned = HashSet::new();
    let mut sum = 0;
    let mut i = 0;
    while assigned.len() != data.plots.len() {
        i += 1;
        let (start, start_tag) = data
            .plots
            .iter()
            .enumerate()
            .find_map(|(i, p)| (!assigned.contains(&i)).then_some((i, *p)))
            .unwrap();
        let mut region = HashSet::from([start]);
        let mut area = 1;
        let mut region_edges = Edge::plot_edges(data.index_to_coord(start));
        let mut to_visit = vec![start];
        while !to_visit.is_empty() {
            // println!("{region_edges:?}");
            let p = to_visit.pop().unwrap();
            let new_plots = data
                .surrounding(p)
                .into_iter()
                .filter_map(|(i, c)| (c.eq(&start_tag) && !region.contains(&i)).then_some(i))
                .collect::<Vec<_>>();
            new_plots.into_iter().for_each(|i| {
                to_visit.push(i);
                region.insert(i);
                area += 1;
                add_plot(&mut region_edges, data.index_to_coord(i));
            })
        }
        println!(
            "Region {}: {}*{}={}\n{:?}\n{:?}\n",
            start_tag,
            area,
            region_edges.len(),
            area * region_edges.len(),
            region_edges,
            region
        );
        sum += area * region_edges.len();
        assigned.extend(region);
        if i > 0 {
            // break;
        }
    }
    sum as u64
}

fn add_plot(edges: &mut Vec<Edge>, coord: (usize, usize)) {
    // println!("b {coord:?}: {edges:?}");
    let new_edges = Edge::plot_edges(coord);
    for edge in new_edges {
        // print!("{edge:?} - ");
        if let Some(i) = edges.iter().position(|e| e.contains(&edge)) {
            let e = edges.remove(i);
            let new_edges = e.sub(&edge);
            // println!("1 {e:?} => {new_edges:?}");
            edges.extend_from_slice(&new_edges);
        } else {
            let mut extendable = edges
                .iter_mut()
                .enumerate()
                .filter(|e| edge.extends(e.1))
                .collect::<Vec<_>>();
            if extendable.is_empty() {
                edges.push(edge);
                // println!("3");
            } else {
                // println!("2");
                let other = extendable.get(1).map(|e| (e.0, e.1.clone()));
                let base = extendable.first_mut().unwrap();
                base.1.extend(&edge);
                let other = match other {
                    Some(e) => {
                        base.1.extend(&e.1);
                        Some(e.0)
                    }
                    None => None,
                };
                if other.is_some() {
                    edges.remove(other.unwrap());
                }
            }
        }
    }
    // println!("a: {edges:?}");
}
