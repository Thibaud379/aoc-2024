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
        eprintln!("Not enough arguments\nUSAGE: PART ./puzzle-6.exe FILE\n\tWhere PART is one of `1` or `2`");
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
            eprint!("Arguments invalid\nUSAGE: PART ./puzzle-6.exe FILE\n\tWhere PART must be one of `1` or `2`");
            return;
        }
    };
    println!("Got result `{sum}`!");
}

#[derive(Clone)]
struct PuzzleData {
    start: (usize, usize),
    lines: Vec<Vec<usize>>,
    cols: Vec<Vec<usize>>,
    width: usize,
}

fn parse_input(lines: std::io::Lines<BufReader<File>>) -> Result<PuzzleData, String> {
    let mut lines = lines.peekable();
    let Some(Ok(width)) = lines.peek() else {
        Err("Could nor parse input file")?
    };
    let width = width.len();
    let (guard, obstacles) = lines
        .map(Result::unwrap)
        .enumerate()
        .map(|(line_idx, line)| {
            (
                line_idx,
                line.find('^'),
                line.match_indices('#').map(|e| e.0).collect::<Vec<_>>(),
            )
        })
        .fold((None, Vec::new()), |(mut g, mut o), (l_idx, g_o, os)| {
            g = g.or(g_o.map(|x| (x, l_idx)));
            o.extend(os.iter().map(|e| (*e, l_idx)));
            (g, o)
        });
    let lines: Vec<Vec<usize>> = obstacles
        .iter()
        .fold(vec![Vec::new(); width], |mut acc, o| {
            acc[o.1].push(o.0);
            acc
        })
        .into_iter()
        .map(|mut v| {
            v.sort_unstable();
            v
        })
        .collect::<Vec<_>>();
    let cols = obstacles
        .iter()
        .fold(vec![Vec::new(); width], |mut acc, o| {
            acc[o.0].push(o.1);
            acc
        })
        .into_iter()
        .map(|mut v| {
            v.sort_unstable();
            v
        })
        .collect::<Vec<_>>();

    Ok(PuzzleData {
        start: guard.unwrap(),
        lines,
        cols,
        width,
    })
}
#[derive(Debug, Clone, Copy, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn part1(data: PuzzleData) -> u64 {
    let mut path = vec![0u8; data.width * data.width];
    let xy_to_idx = |(x, y)| x + data.width * y;
    let mut guard = data.start;
    let mut dir = Dir::Up;
    loop {
        match dir {
            Dir::Up => {
                let obstacle = data.cols[guard.0].binary_search(&guard.1).unwrap_err();

                let obstacle_y = if obstacle == 0 {
                    0
                } else {
                    data.cols[guard.0][obstacle - 1] + 1
                };
                iter::repeat(guard.0)
                    .zip(obstacle_y..=guard.1)
                    .map(xy_to_idx)
                    .for_each(|idx| path[idx] = 1);
                if obstacle_y == 0 {
                    break;
                }
                dir = Dir::Right;
                guard.1 = obstacle_y;
            }
            Dir::Down => {
                let obstacle = data.cols[guard.0].binary_search(&guard.1).unwrap_err();

                let obstacle_y = if obstacle == data.cols[guard.0].len() {
                    data.width - 1
                } else {
                    data.cols[guard.0][obstacle] - 1
                };
                iter::repeat(guard.0)
                    .zip(guard.1..=obstacle_y)
                    .map(xy_to_idx)
                    .for_each(|idx| path[idx] = 1);
                if obstacle_y == data.width - 1 {
                    break;
                }
                dir = Dir::Left;
                guard.1 = obstacle_y;
            }
            Dir::Left => {
                let obstacle = data.lines[guard.1].binary_search(&guard.0).unwrap_err();

                let obstacle_x = if obstacle == 0 {
                    0
                } else {
                    data.lines[guard.1][obstacle - 1] + 1
                };
                (obstacle_x..=guard.0)
                    .zip(iter::repeat(guard.1))
                    .map(xy_to_idx)
                    .for_each(|idx| path[idx] = 1);
                if obstacle_x == 0 {
                    break;
                }
                dir = Dir::Up;
                guard.0 = obstacle_x;
            }
            Dir::Right => {
                let obstacle = data.lines[guard.1].binary_search(&guard.0).unwrap_err();

                let obstacle_x = if obstacle == data.lines[guard.1].len() {
                    data.width - 1
                } else {
                    data.lines[guard.1][obstacle] - 1
                };
                (guard.0..=obstacle_x)
                    .zip(iter::repeat(guard.1))
                    .map(xy_to_idx)
                    .for_each(|idx| path[idx] = 1);
                if obstacle_x == data.width - 1 {
                    break;
                }
                dir = Dir::Down;
                guard.0 = obstacle_x;
            }
        }
    }
    path.into_iter().filter(|e| *e == 1).count() as u64
}
#[derive(Clone)]
struct Guard {
    pos: (usize, usize),
    dir: Dir,
}
enum StepKind {
    Forward(Guard),
    Rotate(Guard),
    Oob,
}

fn next_step(guard: &Guard, data: &PuzzleData) -> StepKind {
    match guard.dir {
        Dir::Up => {
            if data.cols[guard.pos.0]
                .binary_search(&guard.pos.1.saturating_sub(1))
                .is_ok()
            {
                StepKind::Rotate(Guard {
                    pos: guard.pos,
                    dir: Dir::Right,
                })
            } else if guard.pos.1 > 0 {
                StepKind::Forward(Guard {
                    pos: (guard.pos.0, guard.pos.1 - 1),
                    dir: Dir::Up,
                })
            } else {
                StepKind::Oob
            }
        }
        Dir::Down => {
            if data.cols[guard.pos.0]
                .binary_search(&guard.pos.1.saturating_add(1))
                .is_ok()
            {
                StepKind::Rotate(Guard {
                    pos: guard.pos,
                    dir: Dir::Left,
                })
            } else if guard.pos.1 < data.width - 1 {
                StepKind::Forward(Guard {
                    pos: (guard.pos.0, guard.pos.1 + 1),
                    dir: Dir::Down,
                })
            } else {
                StepKind::Oob
            }
        }
        Dir::Left => {
            if data.lines[guard.pos.1]
                .binary_search(&guard.pos.0.saturating_sub(1))
                .is_ok()
            {
                StepKind::Rotate(Guard {
                    pos: guard.pos,
                    dir: Dir::Up,
                })
            } else if guard.pos.0 > 0 {
                StepKind::Forward(Guard {
                    pos: (guard.pos.0 - 1, guard.pos.1),
                    dir: Dir::Left,
                })
            } else {
                StepKind::Oob
            }
        }
        Dir::Right => {
            if data.lines[guard.pos.1]
                .binary_search(&guard.pos.0.saturating_add(1))
                .is_ok()
            {
                StepKind::Rotate(Guard {
                    pos: guard.pos,
                    dir: Dir::Down,
                })
            } else if guard.pos.0 < data.width - 1 {
                StepKind::Forward(Guard {
                    pos: (guard.pos.0 + 1, guard.pos.1),
                    dir: Dir::Right,
                })
            } else {
                StepKind::Oob
            }
        }
    }
}

fn part2(data: PuzzleData) -> u64 {
    const LOOP_SIZE: usize = 100_000;

    let mut guard = Guard {
        pos: data.start,
        dir: Dir::Up,
    };
    let mut possible_obstacles = HashSet::new();
    loop {
        match next_step(&guard, &data) {
            StepKind::Forward(new_guard) => {
                possible_obstacles.insert(new_guard.pos);
                guard = new_guard;
            }
            StepKind::Rotate(new_guard) => guard = new_guard,
            StepKind::Oob => break,
        };
    }

    possible_obstacles.remove(&data.start);
    let mut obstacle_count = 0u64;
    for ob in &possible_obstacles {
        let mut n_data = data.clone();
        n_data.cols[ob.0].push(ob.1);
        n_data.lines[ob.1].push(ob.0);
        n_data.cols[ob.0].sort_unstable();
        n_data.lines[ob.1].sort_unstable();

        let mut guard = Guard {
            pos: data.start,
            dir: Dir::Up,
        };
        obstacle_count += 1;
        for _ in 0..LOOP_SIZE {
            match next_step(&guard, &n_data) {
                StepKind::Forward(new_guard) => {
                    guard = new_guard;
                }
                StepKind::Rotate(new_guard) => guard = new_guard,
                StepKind::Oob => {
                    obstacle_count -= 1;
                    break;
                }
            };
        }
    }
    dbg!(possible_obstacles.len());
    obstacle_count
}
