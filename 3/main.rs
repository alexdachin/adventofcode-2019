use std::collections::HashMap;
use std::fs;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Debug)]
struct Path {
    direction: char,
    length: i32,
}

#[derive(Clone, Copy, Debug)]
struct Cross {
    distance: u32,
    steps: u32,
}

fn follow_path<F>(path: Path, current: &mut Point, mut f: F)
where
    F: FnMut(Point),
{
    if path.direction == 'R' {
        for i in current.y + 1..current.y + path.length + 1 {
            f(Point { x: current.x, y: i });
        }
        current.y += path.length;
        return;
    }

    if path.direction == 'L' {
        for i in (current.y - path.length..current.y).rev() {
            f(Point { x: current.x, y: i });
        }
        current.y -= path.length;
        return;
    }

    if path.direction == 'U' {
        for i in current.x + 1..current.x + path.length + 1 {
            f(Point { x: i, y: current.y });
        }
        current.x += path.length;
        return;
    }

    if path.direction == 'D' {
        for i in (current.x - path.length..current.x).rev() {
            f(Point { x: i, y: current.y });
        }
        current.x -= path.length;
        return;
    }

    panic!("Unknown direction {:?}", path.direction);
}

fn get_wire_paths(wire: &str) -> Vec<Path> {
    wire.split(",")
        .map(|value| Path {
            direction: value.chars().next().unwrap(),
            length: value[1..].parse::<i32>().unwrap(),
        })
        .collect()
}

fn manhattan_distance(p1: Point, p2: Point) -> u32 {
    ((p1.x - p2.x).abs() + (p1.y - p2.y).abs()) as u32
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the input.");

    let wires: Vec<&str> = input.lines().collect();
    let first_wire: Vec<Path> = get_wire_paths(wires[0]);
    let second_wire: Vec<Path> = get_wire_paths(wires[1]);

    let origin = Point { x: 1, y: 1 };
    let mut grid: HashMap<Point, u32> = HashMap::new();

    let mut current = origin;
    let mut steps: u32 = 0;
    for path in first_wire.iter() {
        follow_path(*path, &mut current, |point: Point| {
            steps += 1;
            grid.insert(point, steps);
        })
    }

    let mut crosses: Vec<Cross> = Vec::new();
    current = origin;
    steps = 0;
    for path in second_wire.iter() {
        follow_path(*path, &mut current, |point: Point| {
            steps += 1;
            if grid.contains_key(&point) {
                crosses.push(Cross {
                    distance: manhattan_distance(origin, point),
                    steps: grid.get(&point).unwrap() + steps,
                });
            }
        })
    }

    let least_distance: u32 = crosses.iter().map(|cross| cross.distance).min().unwrap();
    let least_steps: u32 = crosses.iter().map(|cross| cross.steps).min().unwrap();

    println!("Least distance: {:?}", least_distance);
    println!("Least steps: {:?}", least_steps);
}
