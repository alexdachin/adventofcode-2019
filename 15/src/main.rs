use std::collections::HashMap;
use std::fs;

mod intcode;

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
struct Point(isize, isize);

enum AreaItem {
    Wall,
    Empty,
    Oxygen,
}

struct Path {
    steps: usize,
    computer: intcode::Computer,
    point: Point,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the input.");

    let mut original_codes: HashMap<usize, i64> = HashMap::new();
    input
        .trim()
        .split(",")
        .map(|code| code.parse::<i64>().unwrap())
        .enumerate()
        .for_each(|(i, code)| {
            original_codes.insert(i, code);
        });

    const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

    // Part 1
    let mut area: HashMap<Point, AreaItem> = HashMap::new();
    area.insert(Point(0, 0), AreaItem::Empty);

    let mut queued: Vec<Path> = Vec::new();
    queued.push(Path {
        steps: 0,
        computer: intcode::Computer::initialize(&original_codes),
        point: Point(0, 0),
    });

    let computer_from_oxygen: intcode::Computer;

    'path_loop: loop {
        let current = queued.remove(0);

        for direction in 1..5 {
            let new_point = Point(
                current.point.0 + DIRECTIONS[direction - 1].0,
                current.point.1 + DIRECTIONS[direction - 1].1,
            );

            match area.get(&new_point) {
                None => {}
                _ => {
                    continue;
                }
            }

            let mut new_computer = current.computer.clone();
            new_computer.execute();
            new_computer.provide_input(direction as i64);
            new_computer.execute();
            match new_computer.get_output() {
                0 => {
                    area.insert(new_point, AreaItem::Wall);
                }
                1 => {
                    area.insert(new_point.clone(), AreaItem::Empty);
                    queued.push(Path {
                        steps: current.steps + 1,
                        point: new_point.clone(),
                        computer: new_computer,
                    });
                }
                2 => {
                    area.insert(new_point, AreaItem::Oxygen);
                    println!("{} steps to oxygen", current.steps + 1);
                    computer_from_oxygen = current.computer;
                    break 'path_loop;
                }
                _ => panic!("Output range not expected!"),
            }
        }
    }

    // Part 2
    area = HashMap::new();
    area.insert(Point(0, 0), AreaItem::Oxygen);

    queued = Vec::new();
    queued.push(Path {
        steps: 0,
        computer: computer_from_oxygen,
        point: Point(0, 0),
    });

    'path_from_oxygen: loop {
        let current = queued.remove(0);

        for direction in 1..5 {
            let new_point = Point(
                current.point.0 + DIRECTIONS[direction - 1].0,
                current.point.1 + DIRECTIONS[direction - 1].1,
            );

            match area.get(&new_point) {
                None => {}
                _ => {
                    continue;
                }
            }

            let mut new_computer = current.computer.clone();
            new_computer.execute();
            new_computer.provide_input(direction as i64);
            new_computer.execute();
            match new_computer.get_output() {
                0 => {
                    area.insert(new_point, AreaItem::Wall);
                }
                1 => {
                    area.insert(new_point.clone(), AreaItem::Empty);
                    queued.push(Path {
                        steps: current.steps + 1,
                        point: new_point.clone(),
                        computer: new_computer,
                    });
                }
                2 => {
                    area.insert(new_point.clone(), AreaItem::Oxygen);
                    queued.push(Path {
                        steps: current.steps + 1,
                        point: new_point.clone(),
                        computer: new_computer,
                    });
                }
                _ => panic!("Output range not expected!"),
            }
        }

        if queued.is_empty() {
            println!("{} minutes until filled with oxygen.", current.steps + 1);
            break 'path_from_oxygen;
        }
    }
}
