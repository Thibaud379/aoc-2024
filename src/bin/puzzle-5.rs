use std::{
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Not enough arguments\nUSAGE: PART ./puzzle-3.exe FILE\n\tWhere PART is one of `1` or `2`");
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
            eprint!("Arguments invalid\nUSAGE: PART ./puzzle-3.exe FILE\n\tWhere PART must be one of `1` or `2`");
            return;
        }
    };
    println!("Got result `{sum}`!");
}

fn part1(lines: std::io::Lines<BufReader<File>>) -> u64 {
    let mut orderings: HashMap<u64, HashSet<u64>> = HashMap::new();
    let mut updates = Vec::new();
    for l in lines.map(Result::unwrap) {
        if l.len() == 0 {
            continue;
        }
        match l.find('|') {
            Some(i) => {
                orderings
                    .entry(l[..i].parse::<u64>().unwrap())
                    .and_modify(|e| {
                        e.insert(l[(i + 1)..].parse::<u64>().unwrap());
                    })
                    .or_insert(HashSet::from([l[(i + 1)..].parse::<u64>().unwrap()]));
            }
            None => updates.push(
                l.split(',')
                    .map(|n| n.parse::<u64>().unwrap())
                    .collect::<Vec<_>>(),
            ),
        }
    }
    let mut sum = 0;
    for update in updates {
        let mut previous = HashSet::new();
        let mut ok = true;
        for page in update.iter() {
            println!("{previous:?}");
            if orderings
                .get(page)
                .map(|s| s.intersection(&previous).count())
                .is_none_or(|c| c == 0)
            {
                previous.insert(*page);
            } else {
                println!("break");
                ok = false;
                break;
            }
        }
        if ok {
            let middle = update[update.len() / 2];
            println!("{middle}");
            sum += middle
        }
    }
    sum
}

fn swap_error(update: &mut Vec<u64>, orderings: &HashMap<u64, HashSet<u64>>) -> bool {
    let mut all_previous = vec![HashSet::new()];
    let mut error = (0, 0);
    let mut swapped = false;
    for (from, page) in update.iter().enumerate() {
        let previous = all_previous.last_mut().unwrap();
        if orderings
            .get(page)
            .map(|s| s.intersection(&previous).count())
            .is_none_or(|c| c == 0)
        {
            let mut c = previous.clone();
            c.insert(*page);
            all_previous.push(c);
        } else {
            match all_previous.iter().rev().position(|p| {
                orderings
                    .get(page)
                    .map(|s| s.intersection(&p).count())
                    .is_none_or(|c| c == 0)
            }) {
                Some(swap_to) => {
                    error = (from, swap_to);
                }
                None => error = (from, 0),
            }

            swapped = true;
            break;
        }
    }
    if swapped {
        update.swap(error.0, error.1);
    }
    swapped
}

fn part2(lines: std::io::Lines<BufReader<File>>) -> u64 {
    let mut orderings: HashMap<u64, HashSet<u64>> = HashMap::new();
    let mut updates = Vec::new();
    for l in lines.map(Result::unwrap) {
        if l.len() == 0 {
            continue;
        }
        match l.find('|') {
            Some(i) => {
                orderings
                    .entry(l[..i].parse::<u64>().unwrap())
                    .and_modify(|e| {
                        e.insert(l[(i + 1)..].parse::<u64>().unwrap());
                    })
                    .or_insert(HashSet::from([l[(i + 1)..].parse::<u64>().unwrap()]));
            }
            None => updates.push(
                l.split(',')
                    .map(|n| n.parse::<u64>().unwrap())
                    .collect::<Vec<_>>(),
            ),
        }
    }
    let mut sum = 0;
    for mut update in updates {
        if !swap_error(&mut update, &orderings) {
            continue;
        }
        while swap_error(&mut update, &orderings) {}
        let middle = update[update.len() / 2];
        println!("{middle}");
        sum += middle
    }
    sum
}
