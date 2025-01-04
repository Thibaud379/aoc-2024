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
    let sum: u64 = match args[1].as_str() {
        "1" => part1(lines),
        "2" => part2(lines),
        _ => {
            eprint!("Arguments invalid\nUSAGE: PART ./puzzle-6.exe FILE\n\tWhere PART must be one of `1` or `2`");
            return;
        }
    };
    println!("Got result `{sum}`!");
}
#[derive(Debug, Clone, Copy, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn turned_left(&self) -> Self {
        match self {
            Dir::Up => Dir::Left,
            Dir::Down => Dir::Right,
            Dir::Left => Dir::Down,
            Dir::Right => Dir::Up,
        }
    }
}
fn part1(lines: std::io::Lines<BufReader<File>>) -> u64 {
    let mut lines = lines.peekable();
    let Some(Ok(width)) = lines.peek() else {
        return 0;
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
    let lines = obstacles
        .iter()
        .fold(vec![Vec::new(); width], |mut acc, o| {
            acc[o.1].push(o.0);
            acc
        })
        .into_iter()
        .map(|mut v| {
            v.sort();
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
            v.sort();
            v
        })
        .collect::<Vec<_>>();

    let mut path = vec![0u8; width * width];
    let xy_to_idx = |(x, y)| x + width * y;
    let mut guard = guard.unwrap();
    let mut dir = Dir::Up;
    loop {
        println!("{dir:?} - {guard:?}");
        match dir {
            Dir::Up => {
                let obstacle = cols[guard.0].binary_search(&guard.1).unwrap_err();

                let obstacle_y = if obstacle == 0 {
                    0
                } else {
                    cols[guard.0][obstacle - 1] + 1
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
                let obstacle = cols[guard.0].binary_search(&guard.1).unwrap_err();

                let obstacle_y = if obstacle == cols[guard.0].len() {
                    width - 1
                } else {
                    cols[guard.0][obstacle] - 1
                };
                iter::repeat(guard.0)
                    .zip(guard.1..=obstacle_y)
                    .map(xy_to_idx)
                    .for_each(|idx| path[idx] = 1);
                if obstacle_y == width - 1 {
                    break;
                }
                dir = Dir::Left;
                guard.1 = obstacle_y;
            }
            Dir::Left => {
                let obstacle = lines[guard.1].binary_search(&guard.0).unwrap_err();

                let obstacle_x = if obstacle == 0 {
                    0
                } else {
                    lines[guard.1][obstacle - 1] + 1
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
                let obstacle = lines[guard.1].binary_search(&guard.0).unwrap_err();

                let obstacle_x = if obstacle == lines[guard.1].len() {
                    width - 1
                } else {
                    lines[guard.1][obstacle] - 1
                };
                (guard.0..=obstacle_x)
                    .zip(iter::repeat(guard.1))
                    .map(xy_to_idx)
                    .for_each(|idx| path[idx] = 1);
                if obstacle_x == width - 1 {
                    break;
                }
                dir = Dir::Down;
                guard.0 = obstacle_x;
            }
        }
    }
    path.into_iter().filter(|e| *e == 1).count() as u64
}

fn part2(lines: std::io::Lines<BufReader<File>>) -> u64 {
    let mut lines = lines.peekable();
    let Some(Ok(width)) = lines.peek() else {
        return 0;
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
            v.sort();
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
            v.sort();
            v
        })
        .collect::<Vec<_>>();

    // let mut path = vec![0u8; width * width];
    // let xy_to_idx = |(x, y)| x + width * y;
    let mut guard = guard.unwrap();
    let start = guard.clone();
    let mut dir = Dir::Up;
    let mut positions: Vec<(Dir, (usize, usize))> = Vec::new();
    let mut obstacles = HashSet::new();
    const LOOP_CHECKS: usize = 100000;
    let mut tests = 0usize;
    let mut outs = 0usize;
    loop {
        let mut out = false;
        (out, (dir, guard)) = next_pos((dir, guard), &lines, &cols, width).into_info();

        if positions.len() >= 3 {
            let last_pos = positions.last().unwrap().clone();
            let step = match last_pos.0 {
                Dir::Up => (0, -1),
                Dir::Down => (0, 1),
                Dir::Left => (-1, 0),
                Dir::Right => (1, 0),
            };
            let mut temp_positions: Vec<(Dir, (usize, usize))> = Vec::new();
            let mut last_pos = last_pos.1;
            while last_pos != guard {
                last_pos.0 = last_pos.0.saturating_add_signed(step.0);
                last_pos.1 = last_pos.1.saturating_add_signed(step.1);
                let mut obstacle = last_pos.clone();
                obstacle.0 = obstacle.0.saturating_add_signed(step.0);
                obstacle.1 = obstacle.1.saturating_add_signed(step.1);
                if obstacle == start {
                    continue;
                }
                let Ok(mut pos) = next_pos((dir, last_pos.clone()), &lines, &cols, width) else {
                    continue;
                };
                tests += 1;
                temp_positions.push((dir, last_pos));
                temp_positions.push(pos);
                let mut nout = true;
                for _ in 0..LOOP_CHECKS {
                    (nout, pos) = next_pos(pos, &lines, &cols, width).into_info();
                    if !nout {
                        outs += 1;
                        break;
                    }
                    if obstacle != start
                        && (positions.contains(&pos) || temp_positions.contains(&pos))
                    {
                        // println!("BREAK");

                        obstacles.insert(obstacle);
                        break;
                    };
                    temp_positions.push(pos);
                }
            }
        }

        if !out {
            break;
        }
        positions.push((dir, guard.clone()));
        // println!("{positions:?}");
    }
    dbg!(tests, outs);
    obstacles.len().try_into().unwrap()
}

trait InfoResult<T> {
    fn into_info(self) -> (bool, T);
}

impl<T> InfoResult<T> for Result<T, T> {
    fn into_info(self) -> (bool, T) {
        match self {
            Ok(i) => (true, i),
            Err(i) => (false, i),
        }
    }
}

type InfoRes<T> = Result<T, T>;

fn step(
    curr_pos: (Dir, (usize, usize)),
    lines: &Vec<Vec<usize>>,
    cols: &Vec<Vec<usize>>,
    width: usize,
) -> InfoRes<(Dir, (usize, usize))> {
    let mut dir = curr_pos.0;
    let mut guard = curr_pos.1;
    match dir {
        Dir::Up => {
            let obstacle = cols[guard.0].binary_search(&guard.1).unwrap_err();

            let obstacle_y = if obstacle == 0 {
                0
            } else {
                cols[guard.0][obstacle - 1] + 1
            };
            dir = Dir::Right;
            guard.1 = obstacle_y;
        }
        Dir::Down => {
            let obstacle = cols[guard.0].binary_search(&guard.1).unwrap_err();

            let obstacle_y = if obstacle == cols[guard.0].len() {
                width - 1
            } else {
                cols[guard.0][obstacle] - 1
            };

            dir = Dir::Left;
            guard.1 = obstacle_y;
        }
        Dir::Left => {
            let obstacle = lines[guard.1].binary_search(&guard.0).unwrap_err();

            let obstacle_x = if obstacle == 0 {
                0
            } else {
                lines[guard.1][obstacle - 1] + 1
            };

            dir = Dir::Up;
            guard.0 = obstacle_x;
        }
        Dir::Right => {
            let obstacle = lines[guard.1].binary_search(&guard.0).unwrap_err();

            let obstacle_x = if obstacle == lines[guard.1].len() {
                width - 1
            } else {
                lines[guard.1][obstacle] - 1
            };

            dir = Dir::Down;
            guard.0 = obstacle_x;
        }
    }
    if guard.0 == width - 1 || guard.0 == 0 || guard.1 == 0 || guard.1 == width - 1 {
        Err((dir, guard))
    } else {
        Ok((dir, guard))
    }
}

fn next_pos(
    curr_pos: (Dir, (usize, usize)),
    lines: &Vec<Vec<usize>>,
    cols: &Vec<Vec<usize>>,
    width: usize,
) -> InfoRes<(Dir, (usize, usize))> {
    let mut dir = curr_pos.0;
    let mut guard = curr_pos.1;
    match dir {
        Dir::Up => {
            let obstacle = cols[guard.0].binary_search(&guard.1).unwrap_err();

            let obstacle_y = if obstacle == 0 {
                0
            } else {
                cols[guard.0][obstacle - 1] + 1
            };
            dir = Dir::Right;
            guard.1 = obstacle_y;
        }
        Dir::Down => {
            let obstacle = cols[guard.0].binary_search(&guard.1).unwrap_err();

            let obstacle_y = if obstacle == cols[guard.0].len() {
                width - 1
            } else {
                cols[guard.0][obstacle] - 1
            };

            dir = Dir::Left;
            guard.1 = obstacle_y;
        }
        Dir::Left => {
            let obstacle = lines[guard.1].binary_search(&guard.0).unwrap_err();

            let obstacle_x = if obstacle == 0 {
                0
            } else {
                lines[guard.1][obstacle - 1] + 1
            };

            dir = Dir::Up;
            guard.0 = obstacle_x;
        }
        Dir::Right => {
            let obstacle = lines[guard.1].binary_search(&guard.0).unwrap_err();

            let obstacle_x = if obstacle == lines[guard.1].len() {
                width - 1
            } else {
                lines[guard.1][obstacle] - 1
            };

            dir = Dir::Down;
            guard.0 = obstacle_x;
        }
    }
    if guard.0 == width - 1 || guard.0 == 0 || guard.1 == 0 || guard.1 == width - 1 {
        Err((dir, guard))
    } else {
        Ok((dir, guard))
    }
}
