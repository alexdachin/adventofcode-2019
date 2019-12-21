use std::collections::HashMap;
use std::fs;
use std::io;

mod intcode;

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

    let mut computer = intcode::Computer::initialize(&original_codes);
    computer.execute();

    while !computer.halted {
        if computer.waiting_for_input {
            println!("> ");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            println!("Read input: {}", input.trim());

            for character in input.chars() {
                computer.provide_input(character as i64);
                computer.execute();
            }
        }

        if computer.has_pending_output {
            let output = computer.get_output();
            if output < 255 {
                print!("{}", output as u8 as char);
            } else {
                print!("{}", output);
            }
        }

        computer.execute();
    }
}
