use std::collections::{BTreeSet, HashSet, VecDeque};
use std::fs;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Position(u8, u8);

#[derive(Debug, Clone)]
struct Path {
    steps: u32,
    positions: Vec<Position>,
    index: u8,
    keys_collected: BTreeSet<char>,
}

fn get_keys(map: &Vec<Vec<char>>) -> u8 {
    let mut keys: u8 = 0;
    for line in map {
        for column in line {
            if ('a'..='z').contains(&column) {
                keys += 1;
            }
        }
    }
    keys
}

fn get_entrances(map: &Vec<Vec<char>>) -> Vec<Position> {
    let mut entrances: Vec<Position> = Vec::new();
    for (i, line) in map.iter().enumerate() {
        for (j, column) in line.iter().enumerate() {
            if *column == '@' {
                entrances.push(Position(i as u8, j as u8));
            }
        }
    }

    entrances
}

fn shortest(map: &Vec<Vec<char>>) -> u32 {
    const POSIBILITIES: [(i8, i8); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let total_keys = get_keys(&map);
    let mut queue: VecDeque<Path> = VecDeque::new();
    queue.push_back(Path {
        steps: 0,
        positions: get_entrances(&map),
        index: 0,
        keys_collected: BTreeSet::new(),
    });

    let mut visited: HashSet<(Position, BTreeSet<char>)> = HashSet::new();
    let mut debug_collected = 0;
    'path_loop: while !queue.is_empty() {
        let path = queue.pop_front().unwrap();
        let position = path.positions[path.index as usize].clone();

        if !visited.insert((position.clone(), path.keys_collected.clone())) {
            continue;
        }

        let mut keys_collected = path.keys_collected.clone();

        let item = map[position.0 as usize][position.1 as usize];

        if item == '#' {
            continue 'path_loop;
        }

        if ('A'..='Z').contains(&item) && !path.keys_collected.contains(&item.to_ascii_lowercase())
        {
            continue 'path_loop;
        }

        if ('a'..='z').contains(&item) {
            keys_collected.insert(item);
        }

        if keys_collected.len() > debug_collected {
            println!("{} keys collected.", keys_collected.len());
            debug_collected = keys_collected.len();
        }

        if keys_collected.len() == total_keys as usize {
            return path.steps;
        }

        for i in 0..path.positions.len() {
            let mut new_positions = path.positions.clone();
            let original_position = path.positions[i].clone();
            for posibility in &POSIBILITIES {
                new_positions[i] = Position(
                    (original_position.0 as i8 + posibility.0 as i8) as u8,
                    (original_position.1 as i8 + posibility.1 as i8) as u8,
                );
                queue.push_back(Path {
                    steps: path.steps + 1,
                    positions: new_positions.clone(),
                    index: i as u8,
                    keys_collected: keys_collected.clone(),
                });
            }
        }
    }

    panic!("Not found!");
}

fn main() {
    let input = fs::read_to_string("input2.txt").expect("Something went wrong reading the input.");
    let mut map: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        map.push(Vec::new());
        let index = map.len() - 1;
        for column in line.chars() {
            map[index].push(column);
        }
    }

    println!("{}", shortest(&map));
}
