use std::collections::HashMap;
use std::fs;

mod intcode;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Point(i32, i32);

fn is_intersection(map: &HashMap<Point, char>, point: &Point) -> bool {
    for adjacent_position in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
        match map.get(&Point(
            point.0 + adjacent_position.0,
            point.1 + adjacent_position.1,
        )) {
            Some('#') => {}
            _ => {
                return false;
            }
        }
    }

    true
}

fn ignore_outputs(computer: &mut intcode::Computer) {
    while !computer.waiting_for_input {
        computer.execute();
        if computer.has_pending_output {
            computer.get_output();
        }
    }
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

    // Part 1
    let mut computer = intcode::Computer::initialize(&original_codes);
    computer.execute();

    let mut map: HashMap<Point, char> = HashMap::new();
    let mut map_vector: Vec<Vec<char>> = vec![Vec::new()];

    let mut current_point = Point(0, 0);
    while !computer.halted {
        let output = computer.get_output();
        match output {
            118 | 94 | 60 | 62 | 35 | 46 => {
                map.insert(current_point.clone(), output as u8 as char);
                map_vector[current_point.1 as usize].push(output as u8 as char);
                current_point = Point(current_point.0 + 1, current_point.1);
            }
            10 => {
                current_point = Point(0, current_point.1 + 1);
                map_vector.push(Vec::new());
            }
            _ => panic!("Unexpected output."),
        }

        computer.execute();
    }

    let mut sum: u32 = 0;
    for (point, _) in &map {
        if is_intersection(&map, &point) {
            sum += (point.0 * point.1) as u32;
        }
    }

    // Print the map
    for line in map_vector {
        for column in line {
            print!("{}", column);
        }
        println!();
    }

    println!("Part 1: {:?}", sum);

    // Part 2
    original_codes.insert(0, 2);
    let mut computer2 = intcode::Computer::initialize(&original_codes);
    computer2.execute();

    // Computed manually from the printed map
    const INPUTS: [&str; 5] = [
        "A,B,A,C,A,B,C,B,C,B\n",
        "R,8,L,10,L,12,R,4\n",
        "R,8,L,12,R,4,R,4\n",
        "R,8,L,10,R,8\n",
        "n\n",
    ];

    for input in &INPUTS {
        for character in input.chars() {
            ignore_outputs(&mut computer2);
            computer2.provide_input(character as i64);
        }
    }

    let mut last_output = 0;
    while last_output < 255 {
        computer2.execute();
        last_output = computer2.get_output();
    }
    println!("Part 2: {}", last_output);
}
