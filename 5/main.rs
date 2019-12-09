use std::fs;

#[derive(PartialEq)]
enum Opcode {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt,
    Error(usize),
}

enum ParameterMode {
    Position,
    Immediate,
}

struct OpcodeValue {
    opcode: Opcode,
    mode_1: ParameterMode,
    mode_2: ParameterMode,
    mode_3: ParameterMode,
}

fn parse_opcode(value: usize) -> OpcodeValue {
    OpcodeValue {
        opcode: match value % 100 {
            1 => Opcode::Add,
            2 => Opcode::Multiply,
            3 => Opcode::Input,
            4 => Opcode::Output,
            5 => Opcode::JumpIfTrue,
            6 => Opcode::JumpIfFalse,
            7 => Opcode::LessThan,
            8 => Opcode::Equals,
            99 => Opcode::Halt,
            _ => Opcode::Error(value % 100),
        },
        mode_1: match value / 100 % 10 {
            0 => ParameterMode::Position,
            _ => ParameterMode::Immediate,
        },
        mode_2: match value / 1000 % 10 {
            0 => ParameterMode::Position,
            _ => ParameterMode::Immediate,
        },
        mode_3: match value / 10000 % 10 {
            0 => ParameterMode::Position,
            _ => ParameterMode::Immediate,
        },
    }
}

fn get_position(codes: Vec<i32>, position: usize, mode: ParameterMode) -> i32 {
    match mode {
        ParameterMode::Position => codes[codes[position] as usize],
        ParameterMode::Immediate => codes[position],
    }
}

fn compute(original_codes: Vec<i32>, input: i32) {
    let mut codes = original_codes.clone();

    let mut i: usize = 0;
    loop {
        let opcode_value = parse_opcode(codes.clone()[i] as usize);
        match opcode_value.opcode {
            Opcode::Add => {
                let param1 = get_position(codes.clone(), i + 1, opcode_value.mode_1);
                let param2 = get_position(codes.clone(), i + 2, opcode_value.mode_2);
                let output_index = codes[i + 3] as usize;
                codes[output_index] = param1 + param2;
                i += 4;
            }
            Opcode::Multiply => {
                let param1 = get_position(codes.clone(), i + 1, opcode_value.mode_1);
                let param2 = get_position(codes.clone(), i + 2, opcode_value.mode_2);
                let output_index = codes[i + 3] as usize;
                codes[output_index] = param1 * param2;
                i += 4;
            }
            Opcode::Input => {
                let input_index = codes[i + 1] as usize;
                codes[input_index] = input;
                i += 2;
            }
            Opcode::Output => {
                let param1 = get_position(codes.clone(), i + 1, opcode_value.mode_1);
                println!("{}", param1);
                i += 2;
            }
            Opcode::JumpIfTrue => {
                let param1 = get_position(codes.clone(), i + 1, opcode_value.mode_1);
                let param2 = get_position(codes.clone(), i + 2, opcode_value.mode_2) as usize;
                if param1 != 0 {
                    i = param2;
                } else {
                    i += 3;
                }
            }
            Opcode::JumpIfFalse => {
                let param1 = get_position(codes.clone(), i + 1, opcode_value.mode_1);
                let param2 = get_position(codes.clone(), i + 2, opcode_value.mode_2) as usize;
                if param1 == 0 {
                    i = param2;
                } else {
                    i += 3;
                }
            }
            Opcode::LessThan => {
                let param1 = get_position(codes.clone(), i + 1, opcode_value.mode_1);
                let param2 = get_position(codes.clone(), i + 2, opcode_value.mode_2);
                let output_index = codes[i + 3] as usize;
                codes[output_index] = if param1 < param2 { 1 } else { 0 };
                i += 4;
            }
            Opcode::Equals => {
                let param1 = get_position(codes.clone(), i + 1, opcode_value.mode_1);
                let param2 = get_position(codes.clone(), i + 2, opcode_value.mode_2);
                let output_index = codes[i + 3] as usize;
                codes[output_index] = if param1 == param2 { 1 } else { 0 };
                i += 4;
            }
            Opcode::Halt => {
                break;
            }
            Opcode::Error(opcode_number) => {
                println!("Invalid opcode: {}", opcode_number);
                break;
            }
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the input.");
    const SYSTEM_ID: i32 = 5;

    let original_codes: Vec<i32> = input
        .trim()
        .split(",")
        .map(|code| code.parse::<i32>().unwrap())
        .collect();

    compute(original_codes.clone(), SYSTEM_ID);
}
