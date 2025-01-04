use std::{
    collections::HashSet,
    env,
    fs::File,
    io::{BufRead, BufReader},
    iter,
    process::exit,
};
static mut FILE: Option<String> = None;
#[allow(static_mut_refs)]
fn get_lines() -> Vec<String> {
    unsafe {
        let Ok(buf) = File::open(FILE.clone().unwrap()).map(BufReader::new) else {
            eprintln!("Error reading `{FILE:?}`");
            exit(-1);
        };
        buf.lines().map(Result::unwrap).collect::<Vec<_>>()
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Not enough arguments\nUSAGE: PART ./puzzle-4.exe FILE\n\tWhere PART is one of `1` or `2`");
        return;
    }
    unsafe {
        FILE = Some(args[2].clone());
    }

    let sum: u64 = match args[1].as_str() {
        "1" => part1(),
        "2" => part2(),
        _ => {
            eprint!("Arguments invalid\nUSAGE: PART ./puzzle-4.exe FILE\n\tWhere PART must be one of `1` or `2`");
            return;
        }
    };
    println!("Got result `{sum}`!");
}

fn part1() -> u64 {
    let lines_v = get_lines();
    let width = lines_v[0].len();
    let lines = lines_v.iter().cloned();
    let columns = lines_v
        .iter()
        .fold(vec![String::new(); width], |mut acc, line| {
            line.chars().enumerate().for_each(|(i, c)| acc[i].push(c));
            acc
        })
        .into_iter();
    let diag_d = lines_v
        .iter()
        .enumerate()
        .fold(
            vec![String::new(); width * 2 - 1],
            |mut acc, (l_i, line)| {
                line.chars().enumerate().for_each(|(i, c)| {
                    if i + l_i < 2 * width {
                        acc[i + l_i].push(c);
                    }
                });
                acc
            },
        )
        .into_iter();
    let diag_u = lines_v
        .iter()
        .enumerate()
        .fold(
            vec![String::new(); width * 2 - 1],
            |mut acc, (l_i, line)| {
                line.chars().enumerate().for_each(|(i, c)| {
                    if i + width >= l_i {
                        acc[i + width - l_i - 1].push(c);
                    }
                });
                acc
            },
        )
        .into_iter();
    println!("d \n {diag_d:?}");
    println!("u \n {diag_u:?}");
    let all = lines.chain(diag_d).chain(diag_u).chain(columns);
    all.map(|s| {
        // println!("{s}");
        // let idx = s.match_indices("XMAS").collect::<Vec<_>>();
        // let idx_1 = s.match_indices("SAMX").collect::<Vec<_>>();
        // println!("{idx:?} - {idx_1:?}");
        s.matches("XMAS").count() + s.matches("SAMX").count()
    })
    .sum::<usize>() as u64
}

fn part2() -> u64 {
    let lines_v = get_lines();
    let width = lines_v[0].len();
    let diag_d = lines_v
        .iter()
        .enumerate()
        .fold(
            vec![String::new(); width * 2 - 1],
            |mut acc, (l_i, line)| {
                line.chars().enumerate().for_each(|(i, c)| {
                    if i + l_i < 2 * width {
                        acc[i + l_i].push(c);
                    }
                });
                acc
            },
        )
        .into_iter()
        .enumerate()
        .flat_map(|(i, l)| {
            l.match_indices("MAS")
                .chain(l.match_indices("SAM"))
                .map(|t| t.0)
                .zip(iter::repeat(i))
                .collect::<Vec<_>>()
        })
        .collect::<HashSet<_>>();
    let diag_u = lines_v
        .iter()
        .enumerate()
        .fold(
            vec![String::new(); width * 2 - 1],
            |mut acc, (l_i, line)| {
                line.chars().enumerate().for_each(|(i, c)| {
                    if i + width >= l_i {
                        acc[i + width - l_i - 1].push(c);
                    }
                });
                acc
            },
        )
        .into_iter()
        .enumerate()
        .flat_map(|(i, l)| {
            l.match_indices("MAS")
                .chain(l.match_indices("SAM"))
                .map(|t| t.0)
                .zip(iter::repeat(i))
                .collect::<Vec<_>>()
        })
        .collect::<HashSet<_>>();
    println!("{diag_d:?}\n --- \n{diag_u:?}");
    diag_d
        .iter()
        .filter(|(x, y)| {
            let (_x1, y1) = if *y > width - 1 {
                (width - 1 - x, x + y - width + 1)
            } else {
                (y - x, *x)
            };
            let Some(x1) = _x1.checked_sub(2) else {
                return false;
            };
            print!("({x},{y})->({_x1},{y1})");
            let x2 = x1.min(y1);
            let y2 = width - 1 + x1 - y1;
            print!("->({x1},{y1})->({x2},{y2})");
            let b = diag_u.contains(&(x2, y2));
            println!(" => {b}");
            b
        })
        .count() as u64
}
