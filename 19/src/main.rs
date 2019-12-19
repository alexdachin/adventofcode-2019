use std::collections::HashMap;
use std::fs;

mod intcode;

fn is_pulled(x: u32, y: u32, original_computer: &intcode::Computer) -> bool {
    let mut computer = original_computer.clone();
    computer.execute();
    computer.provide_input(x as i64);
    computer.execute();
    computer.provide_input(y as i64);
    computer.execute();
    computer.get_output() == 1
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

    let original_computer = intcode::Computer::initialize(&original_codes);

    // Part 1
    let mut points_affected: u32 = 0;
    for i in 0..50 {
        for j in 0..50 {
            if is_pulled(i, j, &original_computer) {
                points_affected += 1;
            }
        }
    }
    println!("Part 1: {} points affected", points_affected);

    // Part 2
    let mut current_y: u32 = 100;
    let mut base_x: u32 = 0;
    loop {
        let mut current_x = base_x;

        while !is_pulled(current_x, current_y, &original_computer) {
            current_x += 1;
        }

        base_x = current_x;

        if is_pulled(current_x + 99, current_y - 99, &original_computer) {
            println!("Part 2: {}", current_x * 10000 + (current_y - 99));
            break;
        }

        current_y += 1;
    }
}
