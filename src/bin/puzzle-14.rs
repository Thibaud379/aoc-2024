use std::{
    env,
    fs::File as FileFs,
    io::{stdin, BufRead, BufReader},
    ops::{Add, Mul, Rem},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Not enough arguments\nUSAGE: PART ./puzzle-14.exe FILE\n\tWhere PART is one of `1` or `2`");
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
            eprint!("Arguments invalid\nUSAGE: PART ./puzzle-14.exe FILE\n\tWhere PART must be one of `1` or `2`");
            return;
        }
    };
    println!("Got result `{sum}`!");
}

#[derive(Clone, Debug, Default, Copy)]
struct Vec2D {
    x: isize,
    y: isize,
}

#[derive(Clone, Debug, Default)]
struct Robot {
    p: Vec2D,
    v: Vec2D,
}
#[derive(Clone, Debug)]
struct PuzzleData {
    robots: Vec<Robot>,
}

fn parse_input(lines: std::io::Lines<BufReader<FileFs>>) -> PuzzleData {
    let robots = lines
        .map(Result::unwrap)
        .map(|l| {
            let p = l.find('=').unwrap();
            let cp = l.find(',').unwrap();
            let sep = l.find(' ').unwrap();
            let v = l[cp..].find('=').unwrap() + cp;
            let cv = l[v..].find(',').unwrap() + v;
            Robot {
                p: Vec2D {
                    x: l[(p + 1)..cp].parse().unwrap(),
                    y: l[(cp + 1)..sep].parse().unwrap(),
                },
                v: Vec2D {
                    x: l[(v + 1)..cv].parse().unwrap(),
                    y: l[(cv + 1)..].parse().unwrap(),
                },
            }
        })
        .collect();
    PuzzleData { robots }
}

impl Add for Vec2D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul<isize> for Vec2D {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Vec2D {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Rem for Vec2D {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Vec2D {
            x: self.x.rem_euclid(rhs.x),
            y: self.y.rem_euclid(rhs.y),
        }
    }
}

fn part1(mut data: PuzzleData) -> u64 {
    // example
    // const SIZE: Vec2D = Vec2D { x: 11, y: 7 };
    // input
    const SIZE: Vec2D = Vec2D { x: 101, y: 103 };
    const TIME: isize = 100;
    for rob in &mut data.robots {
        let v = rob.v * TIME;
        let s = rob.p + v;
        let r = s % SIZE;
        rob.p = r;
    }
    let mut quadrants = [0, 0, 0, 0];
    let mids = Vec2D {
        x: SIZE.x / 2,
        y: SIZE.y / 2,
    };
    for rob in data.robots {
        if rob.p.x < mids.x && rob.p.y < mids.y {
            quadrants[0] += 1;
        } else if rob.p.x > mids.x && rob.p.y < mids.y {
            quadrants[1] += 1;
        } else if rob.p.x > mids.x && rob.p.y > mids.y {
            quadrants[2] += 1;
        } else if rob.p.x < mids.x && rob.p.y > mids.y {
            quadrants[3] += 1;
        }
    }

    quadrants.into_iter().product::<i32>() as u64
}

fn part2(mut data: PuzzleData) -> u64 {
    // example
    // const SIZE: Vec2D = Vec2D { x: 11, y: 7 };
    // input
    const SIZE: Vec2D = Vec2D { x: 101, y: 103 };
    const TIME: isize = 1;
    let mut i = 0;
    loop {
        let mut array = [[' '; SIZE.x as usize]; SIZE.y as usize];
        let mut overlap = 0f64;
        for rob in &mut data.robots {
            let v = rob.v * TIME;
            let s = rob.p + v;
            let r = s % SIZE;
            rob.p = r;
            if array[rob.p.y as usize][rob.p.x as usize] == '■' {
                overlap += 1f64;
            }
            array[rob.p.y as usize][rob.p.x as usize] = '■';
        }
        if overlap / (data.robots.len() as f64) < 0.005f64 {
            for x in 0..SIZE.x as usize {
                for y in 0..SIZE.y as usize {
                    print!("{}", array[y][x]);
                }
                println!();
            }

            println!("{i}");
            let mut s = String::new();
            stdin().read_line(&mut s).unwrap();
        }
        i += 1;
    }
}
