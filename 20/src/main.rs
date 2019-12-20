use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Position(u32, u32);

#[derive(Debug, Clone)]
struct Path {
    steps: u32,
    position: Position,
}

#[derive(Debug, Clone)]
struct Path2 {
    steps: u32,
    position: Position,
    nest: u32,
}

#[derive(Debug)]
enum NamedTile {
    Position(Position),
    Portal(Position, Position),
}

fn get_named_tiles(
    map: &Vec<Vec<char>>,
) -> (HashMap<String, NamedTile>, HashMap<Position, String>) {
    let mut named_tiles: HashMap<String, NamedTile> = HashMap::new();
    let mut position_to_name: HashMap<Position, String> = HashMap::new();
    for i in 2..map.len() - 2 {
        for j in 2..map[i].len() - 2 {
            match map[i][j] {
                '.' => {}
                _ => continue,
            }

            let current_position = Position(i as u32, j as u32);
            let name_positions: [(Position, Position); 4] = [
                (
                    Position((i - 2) as u32, j as u32),
                    Position((i - 1) as u32, j as u32),
                ),
                (
                    Position(i as u32, (j - 2) as u32),
                    Position(i as u32, (j - 1) as u32),
                ),
                (
                    Position((i + 1) as u32, j as u32),
                    Position((i + 2) as u32, j as u32),
                ),
                (
                    Position(i as u32, (j + 1) as u32),
                    Position(i as u32, (j + 2) as u32),
                ),
            ];

            for positions in &name_positions {
                let char1 = map[(positions.0).0 as usize][(positions.0).1 as usize];
                let char2 = map[(positions.1).0 as usize][(positions.1).1 as usize];
                if char1.is_ascii_uppercase() && char2.is_ascii_uppercase() {
                    let name = format!("{}{}", char1, char2);
                    match named_tiles.get(&name.to_string()) {
                        None => {
                            named_tiles.insert(
                                name.clone(),
                                NamedTile::Position(current_position.clone()),
                            );
                            position_to_name.insert(current_position.clone(), name);
                        }
                        Some(NamedTile::Position(previous_position)) => {
                            named_tiles.insert(
                                name.clone(),
                                NamedTile::Portal(
                                    current_position.clone(),
                                    previous_position.clone(),
                                ),
                            );
                            position_to_name.insert(current_position.clone(), name);
                        }
                        Some(NamedTile::Portal(position1, position2)) => {
                            panic!(
                                "Portal already found between {:?} and {:?}!",
                                position1, position2
                            );
                        }
                    }
                }
            }
        }
    }
    (named_tiles, position_to_name)
}

fn get_named_tile_position(named_tile: &NamedTile) -> Position {
    match named_tile {
        NamedTile::Position(position) => position.clone(),
        _ => panic!("Named tile is not a position!"),
    }
}

fn shortest1(map: &Vec<Vec<char>>) -> u32 {
    const POSIBILITIES: [(i8, i8); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let (named_tiles, position_to_name) = get_named_tiles(&map);

    let start = get_named_tile_position(named_tiles.get("AA").unwrap());
    let end = get_named_tile_position(named_tiles.get("ZZ").unwrap());

    let mut queue: VecDeque<Path> = VecDeque::new();
    queue.push_back(Path {
        steps: 0,
        position: start,
    });

    let mut visited: HashSet<Position> = HashSet::new();
    'path_loop: while !queue.is_empty() {
        let path = queue.pop_front().unwrap();

        if !visited.insert(path.position.clone()) {
            continue;
        }

        let mut new_steps = 1;
        let position = match position_to_name.get(&path.position) {
            None => path.position.clone(),
            Some(name) => {
                let named_tile = named_tiles.get(name);
                match named_tile {
                    None => panic!("No named tile at position {:?}!", path.position),
                    Some(NamedTile::Portal(position1, position2)) => {
                        if path.position == *position1 {
                            position2.clone()
                        } else {
                            position1.clone()
                        }
                    }
                    _ => path.position.clone(),
                }
            }
        };

        if position != path.position {
            new_steps += 1;
            visited.insert(position.clone());
        }

        let item = map[position.0 as usize][position.1 as usize];

        match item {
            '.' => {}
            _ => continue,
        }

        if path.position == end {
            return path.steps;
        }

        for posibility in &POSIBILITIES {
            let new_position = Position(
                (position.0 as i32 + posibility.0 as i32) as u32,
                (position.1 as i32 + posibility.1 as i32) as u32,
            );
            queue.push_back(Path {
                steps: path.steps + new_steps,
                position: new_position.clone(),
            });
        }
    }

    panic!("Not found!");
}

fn shortest2(map: &Vec<Vec<char>>) -> u32 {
    const POSIBILITIES: [(i8, i8); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let (named_tiles, position_to_name) = get_named_tiles(&map);

    let start = get_named_tile_position(named_tiles.get("AA").unwrap());
    let end = get_named_tile_position(named_tiles.get("ZZ").unwrap());

    let mut queue: VecDeque<Path2> = VecDeque::new();
    queue.push_back(Path2 {
        steps: 0,
        position: start,
        nest: 0,
    });

    let mut visited: HashSet<(Position, u32)> = HashSet::new();
    'path_loop: while !queue.is_empty() {
        let path = queue.pop_front().unwrap();

        if !visited.insert((path.position.clone(), path.nest)) {
            continue;
        }

        let is_outer = path.position.0 == 2
            || path.position.1 == 2
            || path.position.0 == map.len() as u32 - 2 - 1
            || path.position.1 == map[0].len() as u32 - 2 - 1;

        let mut new_steps = 1;
        let mut new_nest: i8 = 0;
        let position = if path.nest == 0 && is_outer {
            path.position.clone()
        } else {
            match position_to_name.get(&path.position) {
                None => path.position.clone(),
                Some(name) => {
                    let named_tile = named_tiles.get(name);
                    match named_tile {
                        None => panic!("No named tile at position {:?}!", path.position),
                        Some(NamedTile::Portal(position1, position2)) => {
                            if path.position == *position1 {
                                position2.clone()
                            } else {
                                position1.clone()
                            }
                        }
                        _ => path.position.clone(),
                    }
                }
            }
        };

        if position != path.position {
            new_steps += 1;
            new_nest += if is_outer { -1 } else { 1 };
            visited.insert((
                position.clone(),
                (path.nest as i32 + new_nest as i32) as u32,
            ));
        }

        let item = map[position.0 as usize][position.1 as usize];

        match item {
            '.' => {}
            _ => continue,
        }

        if path.position == end && path.nest == 0 {
            return path.steps;
        }

        for posibility in &POSIBILITIES {
            let new_position = Position(
                (position.0 as i32 + posibility.0 as i32) as u32,
                (position.1 as i32 + posibility.1 as i32) as u32,
            );
            queue.push_back(Path2 {
                steps: path.steps + new_steps,
                position: new_position.clone(),
                nest: (path.nest as i32 + new_nest as i32) as u32,
            });
        }
    }

    panic!("Not found!");
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the input.");
    let mut map: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        map.push(Vec::new());
        let index = map.len() - 1;
        for column in line.chars() {
            map[index].push(column);
        }
    }

    println!("Part 1: {:?}", shortest1(&map,));
    println!("Part 2: {:?}", shortest2(&map,));
}
