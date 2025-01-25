use std::{
    collections::HashSet,
    env,
    fmt::Display,
    fs::File as FileFs,
    hash::RandomState,
    io::{BufRead, BufReader},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Not enough arguments\nUSAGE: PART ./puzzle-15.exe FILE\n\tWhere PART is one of `1` or `2`");
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
            eprint!("Arguments invalid\nUSAGE: PART ./puzzle-15.exe FILE\n\tWhere PART must be one of `1` or `2`");
            return;
        }
    };
    println!("Got result `{sum}`!");
}

#[derive(Clone, Debug, Copy)]
enum Tile {
    Wall,
    Free,
    Box,
}
#[derive(Clone, Debug, Copy)]
enum TileD {
    Wall,
    Free,
    BoxLeft,
    BoxRight,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' | '@' => Self::Free,
            '#' => Self::Wall,
            'O' => Self::Box,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug, Copy)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Display for TileD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TileD::Wall => f.write_str("#"),
            TileD::Free => f.write_str("."),
            TileD::BoxLeft => f.write_str("["),
            TileD::BoxRight => f.write_str("]"),
        }
    }
}
impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Wall => f.write_str("#"),
            Tile::Free => f.write_str("."),
            Tile::Box => f.write_str("O"),
        }
    }
}

impl Dir {
    fn next(&self, data: &PuzzleData) -> usize {
        let index = data.position;
        match self {
            Dir::Up => index - data.width,
            Dir::Right => index + 1,
            Dir::Down => index + data.width,
            Dir::Left => index - 1,
        }
    }
    fn next_raw(&self, index: usize, width: usize) -> usize {
        match self {
            Dir::Up => index - width,
            Dir::Right => index + 1,
            Dir::Down => index + width,
            Dir::Left => index - 1,
        }
    }
}

impl From<char> for Dir {
    fn from(value: char) -> Self {
        match value {
            '<' => Self::Left,
            '>' => Self::Right,
            '^' => Self::Up,
            'v' => Self::Down,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug)]
struct PuzzleData {
    terrain: Vec<Tile>,
    width: usize,
    position: usize,
    moves: Vec<Dir>,
}

impl PuzzleData {
    fn index_to_coord(&self, index: usize) -> (usize, usize) {
        (index % self.width, index / self.width)
    }

    fn lines(&mut self) -> Vec<&mut [Tile]> {
        self.terrain.chunks_mut(self.width).collect()
    }

    fn cols(&mut self) -> Vec<Vec<&mut Tile>> {
        let w = self.width;
        let mut out = Vec::new();
        for x in 0..w {
            let mut col = Vec::new();
            for y in 0..w {
                let r = self.terrain.as_mut_ptr_range();
                let v = r.start.wrapping_add(x + y * w);
                unsafe {
                    col.push(v.as_mut().unwrap());
                }
            }
            out.push(col);
        }
        out
    }
}

fn parse_input(mut lines: std::io::Lines<BufReader<FileFs>>) -> PuzzleData {
    let mut width = 0;
    let mut terrain = Vec::new();
    let mut start = 0;
    let mut moves = Vec::new();
    let mut reading_moves = false;
    let mut i = 0;
    while let Some(Ok(l)) = lines.next() {
        if l.is_empty() {
            reading_moves = true;
            continue;
        }
        if !reading_moves {
            width = l.len();
            if let Some(v) = l.find('@').map(|x| (x + i * width)) {
                start = v;
            };
            terrain.extend(l.chars().map(Tile::from));
            i += 1;
        } else {
            moves.extend(l.chars().map(Dir::from));
        }
    }

    PuzzleData {
        terrain,
        width,
        position: start,
        moves,
    }
}

fn part1(mut data: PuzzleData) -> u64 {
    let moves = data.moves.clone();
    let width = data.width;
    for m in moves {
        let next = m.next(&data);
        match &data.terrain[next] {
            Tile::Wall => (),
            Tile::Free => data.position = next,
            Tile::Box => {
                let (col_id, row_id) = data.index_to_coord(data.position);
                match m {
                    Dir::Up | Dir::Down => {
                        let col = &mut data.cols()[col_id];
                        let mut empty_spot = None;
                        if matches!(m, Dir::Down) {
                            for (i, t) in col[(row_id + 1)..].iter().enumerate() {
                                match t {
                                    Tile::Wall => break,
                                    Tile::Free => {
                                        empty_spot = Some(i + row_id + 1);
                                        break;
                                    }
                                    Tile::Box => continue,
                                }
                            }
                        } else {
                            for (i, t) in col[..row_id].iter().enumerate().rev() {
                                match t {
                                    Tile::Wall => break,
                                    Tile::Free => {
                                        empty_spot = Some(i);
                                        break;
                                    }
                                    Tile::Box => continue,
                                }
                            }
                        }
                        if let Some(i) = empty_spot {
                            *col[i] = Tile::Box;
                            *col[next / width] = Tile::Free;

                            data.position = next;
                        }
                    }
                    Dir::Right | Dir::Left => {
                        let row = &mut data.lines()[row_id];
                        let mut empty_spot = None;
                        if matches!(m, Dir::Right) {
                            for (i, t) in row[(col_id + 1)..].iter().enumerate() {
                                match t {
                                    Tile::Wall => break,
                                    Tile::Free => {
                                        empty_spot = Some(i + col_id + 1);
                                        break;
                                    }
                                    Tile::Box => continue,
                                }
                            }
                        } else {
                            for (i, t) in row[..col_id].iter().enumerate().rev() {
                                match t {
                                    Tile::Wall => break,
                                    Tile::Free => {
                                        empty_spot = Some(i);
                                        break;
                                    }
                                    Tile::Box => continue,
                                }
                            }
                        }
                        if let Some(i) = empty_spot {
                            row[i] = Tile::Box;
                            row[next % width] = Tile::Free;
                            data.position = next;
                        }
                    }
                }
            }
        }
        // let curr = data.index_to_coord(data.position);
        // for line in data.lines().iter().enumerate() {
        //     for t in line.1.iter().enumerate() {
        //         if line.0 == curr.1 && t.0 == curr.0 {
        //             match t.1 {
        //                 Tile::Wall => print!("X"),
        //                 Tile::Free => print!("@"),
        //                 Tile::Box => print!("8"),
        //             }
        //         } else {
        //             print!("{}", t.1);
        //         }
        //     }
        //     println!("");
        // }
    }
    let mut sum = 0;
    for line in data.lines().iter().enumerate() {
        for t in line
            .1
            .iter()
            .enumerate()
            .filter(|t| matches!(t.1, Tile::Box))
        {
            sum += line.0 * 100 + t.0;
        }
    }

    sum as u64
}

fn lines(terrain: &Vec<TileD>, height: usize) -> Vec<&[TileD]> {
    terrain.chunks(height * 2).collect::<Vec<_>>()
}

fn part2(data: PuzzleData) -> u64 {
    let height = data.width;
    let width = height * 2;
    let mut terrain: Vec<TileD> = data
        .terrain
        .iter()
        .flat_map(|t| match t {
            Tile::Wall => [TileD::Wall; 2],
            Tile::Free => [TileD::Free; 2],
            Tile::Box => [TileD::BoxLeft, TileD::BoxRight],
        })
        .collect();
    let moves = data.moves.clone();
    let mut position = data.position * 2;
    // let mut i = 0;
    for m in moves {
        // i += 1;
        // if i % 50 == 0 {
        //     let mut s = String::new();
        //     stdin().read_line(&mut s).unwrap();
        //     println!("\n\n\n\n\n\n\n");
        // }
        let next = m.next_raw(position, width);
        match &terrain[next] {
            TileD::Wall => (),
            TileD::Free => position = next,
            TileD::BoxLeft | TileD::BoxRight => {
                let (col_id, row_id) = (position % width, position / width);
                match m {
                    Dir::Right | Dir::Left => {
                        let row = lines(&terrain, height)[row_id];
                        let mut empty_spot = None;
                        if matches!(m, Dir::Right) {
                            for (i, t) in row[(col_id + 1)..].iter().enumerate() {
                                match t {
                                    TileD::Wall => break,
                                    TileD::Free => {
                                        empty_spot = Some(i + col_id + 1);
                                        break;
                                    }
                                    TileD::BoxLeft | TileD::BoxRight => continue,
                                }
                            }
                        } else {
                            // println!("{:?}", &row[..col_id]);
                            for (i, t) in row[..col_id].iter().enumerate().rev() {
                                match t {
                                    TileD::Wall => break,
                                    TileD::Free => {
                                        empty_spot = Some(i);
                                        break;
                                    }
                                    TileD::BoxLeft | TileD::BoxRight => continue,
                                }
                            }
                        }
                        // println!("{empty_spot:?}");
                        if let Some(i) = empty_spot {
                            let offset = row_id * width;
                            if matches!(m, Dir::Right) {
                                for id in ((next % width) + 1)..i {
                                    terrain.swap(offset + id, offset + id - 1);
                                }
                                terrain.swap(offset + i, offset + next % width);
                            } else {
                                // println!("{i}..{}\n{terrain:?}", (next % width));
                                for id in i..(next % width) {
                                    terrain.swap(offset + id, offset + id + 1);
                                }
                                // println!("{terrain:?}");
                            };

                            position = next;
                        }
                    }
                    Dir::Up | Dir::Down => {
                        let mut all_tiles: HashSet<usize> = HashSet::new();
                        let mut tiles_to_move = Vec::new();
                        tiles_to_move.push(next);
                        if matches!(&terrain[next], TileD::BoxRight) {
                            tiles_to_move.push(next - 1);
                        } else {
                            tiles_to_move.push(next + 1);
                        }
                        all_tiles.extend(&tiles_to_move);
                        while tiles_to_move.iter().any(|p| {
                            matches!(terrain[m.next_raw(*p, width)], TileD::BoxLeft)
                                || matches!(terrain[m.next_raw(*p, width)], TileD::BoxRight)
                        }) {
                            let left: Vec<_> = tiles_to_move
                                .iter()
                                .copied()
                                .enumerate()
                                .filter(|t| {
                                    matches!(terrain[m.next_raw(t.1, width)], TileD::BoxLeft)
                                })
                                .collect();
                            let right: Vec<_> = tiles_to_move
                                .iter()
                                .copied()
                                .enumerate()
                                .filter(|t| {
                                    matches!(terrain[m.next_raw(t.1, width)], TileD::BoxRight)
                                })
                                .collect();
                            tiles_to_move.retain(|t| {
                                matches!(terrain[m.next_raw(*t, width)], TileD::Free)
                                    || matches!(terrain[m.next_raw(*t, width)], TileD::Wall)
                            });
                            for (_id, t_id) in left {
                                tiles_to_move
                                    .extend([m.next_raw(t_id, width), m.next_raw(t_id, width) + 1]);
                            }
                            for (_id, t_id) in right {
                                tiles_to_move
                                    .extend([m.next_raw(t_id, width), m.next_raw(t_id, width) - 1]);
                            }

                            tiles_to_move = HashSet::<_, RandomState>::from_iter(tiles_to_move)
                                .into_iter()
                                .collect();
                            all_tiles.extend(&tiles_to_move);
                        }
                        if tiles_to_move
                            .iter()
                            .all(|t| matches!(terrain[m.next_raw(*t, width)], TileD::Free))
                        {
                            position = next;
                            let mut tiles: Vec<_> = all_tiles.into_iter().collect();
                            tiles.sort_unstable();
                            if matches!(m, Dir::Down) {
                                tiles.reverse();
                            }
                            for t in tiles {
                                terrain.swap(t, m.next_raw(t, width));
                            }
                        }
                        println!("{tiles_to_move:?}");
                    }
                }
            }
        }
        // let curr = (position % width, position / width);
        // for line in lines(&terrain, height).into_iter().enumerate() {
        //     for t in line.1.into_iter().enumerate() {
        //         if line.0 == curr.1 && t.0 == curr.0 {
        //             match t.1 {
        //                 TileD::Wall => print!("X"),
        //                 TileD::Free => print!("@"),
        //                 TileD::BoxLeft => print!("{{"),
        //                 TileD::BoxRight => print!("}}"),
        //             }
        //         } else {
        //             print!("{}", t.1);
        //         }
        //     }
        //     println!("");
        // }
    }

    let mut sum = 0;
    for line in lines(&terrain, height).into_iter().enumerate() {
        for t in line
            .1
            .iter()
            .enumerate()
            .filter(|t| matches!(t.1, TileD::BoxLeft))
        {
            sum += line.0 * 100 + t.0;
        }
    }

    sum as u64
}
