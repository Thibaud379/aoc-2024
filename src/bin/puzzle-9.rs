use std::{
    collections::HashSet,
    env,
    fs::File as FileFs,
    io::{BufRead, BufReader},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Not enough arguments\nUSAGE: PART ./puzzle-9.exe FILE\n\tWhere PART is one of `1` or `2`");
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
            eprint!("Arguments invalid\nUSAGE: PART ./puzzle-9.exe FILE\n\tWhere PART must be one of `1` or `2`");
            return;
        }
    };
    println!("Got result `{sum}`!");
}

#[derive(Clone, Debug, PartialEq)]
struct Block {
    id: Option<usize>,
    size: usize,
    pos: usize,
}
#[derive(Clone, Debug)]
struct PuzzleData {
    files: Vec<Block>,
}

fn parse_input(mut lines: std::io::Lines<BufReader<FileFs>>) -> PuzzleData {
    let line = lines.next().unwrap().unwrap();
    let mut files = Vec::new();
    let mut id = Some(0);
    let mut pos = 0;
    for (is_file, size) in line
        .chars()
        .enumerate()
        .map(|(i, v)| (i % 2 == 0, v.to_digit(10).unwrap() as usize))
    {
        if is_file {
            files.push(Block { id, size, pos });
            id = Some(id.unwrap() + 1);
        } else {
            files.push(Block {
                id: None,
                size,
                pos,
            });
        }
        pos += size;
    }
    PuzzleData { files }
}
fn get_block(index: usize, data: &PuzzleData) -> &Block {
    let id = data.files.binary_search_by(|v| v.pos.cmp(&index));
    match id {
        Ok(i) => &data.files[i],
        Err(i) => &data.files[i - 1],
    }
}
fn part1(data: PuzzleData) -> u64 {
    let mut sum = 0;
    let mut lpos = 0;
    let rpos = data.files.last().unwrap();
    let mut rpos = rpos.pos + rpos.size - 1;
    while rpos > lpos {
        let block = get_block(lpos, &data);
        if let Some(id) = block.id {
            sum += lpos * id;
            lpos += 1;
        } else {
            let rblock = get_block(rpos, &data);
            if block == rblock {
                break;
            }
            if let Some(id) = rblock.id {
                sum += lpos * id;
            } else {
                rpos = rblock.pos - 1;
                let rgblock = get_block(rpos, &data);
                sum += lpos * rgblock.id.unwrap();
            }
            lpos += 1;
            rpos -= 1;
        }
    }
    let last_block = get_block(lpos, &data);
    if let Some(id) = last_block.id {
        sum += lpos * id;
    }
    sum as u64
}

fn part2(data: PuzzleData) -> u64 {
    let mut frees = data
        .files
        .iter()
        .filter(|b| b.id.is_none())
        .cloned()
        .collect::<Vec<_>>();
    let mut files = Vec::new();

    let get_file = |id: usize| &data.files[id * 2];
    let mut file_id = data.files.last().unwrap().id.unwrap() + 1;
    while file_id > 0 {
        file_id -= 1;
        let file = get_file(file_id);
        let Some(free_space) = frees
            .iter_mut()
            .filter(|f| f.size >= file.size && f.pos < file.pos)
            .next()
        else {
            files.push(file.clone());
            continue;
        };
        let mut n_file = file.clone();
        n_file.pos = free_space.pos;
        files.push(n_file);
        free_space.pos += file.size;
        free_space.size -= file.size;
    }
    files.sort_by(|a, b| a.pos.cmp(&b.pos));
    println!("{files:?}");
    let mut sum = 0;
    for f in files {
        for i in f.pos..(f.pos + f.size) {
            sum += i * f.id.unwrap();
        }
    }
    sum as u64
}
