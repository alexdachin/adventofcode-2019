use permutohedron::heap_recursive;
use std::fs;

#[derive(Debug, PartialEq)]
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

#[derive(Debug)]
enum ParameterMode {
    Position,
    Immediate,
}

#[derive(Debug)]
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

fn get_position(codes: &Vec<i32>, position: usize, mode: ParameterMode) -> i32 {
    match mode {
        ParameterMode::Position => codes[codes[position] as usize],
        ParameterMode::Immediate => codes[position],
    }
}

#[derive(Debug)]
struct Computer {
    codes: Vec<i32>,
    instruction_pointer: usize,
    halted: bool,
    waiting_for_input: bool,
    has_pending_output: bool,
}

impl Computer {
    fn initialize(codes: &Vec<i32>) -> Computer {
        Computer {
            codes: codes.clone(),
            instruction_pointer: 0,
            halted: false,
            waiting_for_input: false,
            has_pending_output: false,
        }
    }

    fn provide_input(&mut self, input: i32) {
        let opcode_value = parse_opcode(self.codes[self.instruction_pointer] as usize);
        match opcode_value.opcode {
            Opcode::Input => {
                let input_index = self.codes[self.instruction_pointer + 1] as usize;
                self.codes[input_index] = input;
                self.waiting_for_input = false;
                self.instruction_pointer += 2;
            }
            _ => {
                panic!("Input not expected");
            }
        }
    }

    fn get_output(&mut self) -> i32 {
        let opcode_value = parse_opcode(self.codes[self.instruction_pointer] as usize);
        match opcode_value.opcode {
            Opcode::Output => {
                let param1 = get_position(
                    &self.codes,
                    self.instruction_pointer + 1,
                    opcode_value.mode_1,
                );
                self.instruction_pointer += 2;
                self.has_pending_output = false;
                param1
            }
            _ => {
                panic!("Output not expected");
            }
        }
    }

    fn execute(&mut self) {
        loop {
            let i = self.instruction_pointer;
            let opcode_value = parse_opcode(self.codes[i] as usize);

            match opcode_value.opcode {
                Opcode::Add => {
                    let param1 = get_position(&self.codes, i + 1, opcode_value.mode_1);
                    let param2 = get_position(&self.codes, i + 2, opcode_value.mode_2);
                    let output_index = self.codes[i + 3] as usize;
                    self.codes[output_index] = param1 + param2;
                    self.instruction_pointer += 4;
                }
                Opcode::Multiply => {
                    let param1 = get_position(&self.codes, i + 1, opcode_value.mode_1);
                    let param2 = get_position(&self.codes, i + 2, opcode_value.mode_2);
                    let output_index = self.codes[i + 3] as usize;
                    self.codes[output_index] = param1 * param2;
                    self.instruction_pointer += 4;
                }
                Opcode::Input => {
                    self.waiting_for_input = true;
                    break;
                }
                Opcode::Output => {
                    self.has_pending_output = true;
                    break;
                }
                Opcode::JumpIfTrue => {
                    let param1 = get_position(&self.codes, i + 1, opcode_value.mode_1);
                    let param2 = get_position(&self.codes, i + 2, opcode_value.mode_2) as usize;
                    if param1 != 0 {
                        self.instruction_pointer = param2;
                    } else {
                        self.instruction_pointer += 3;
                    }
                }
                Opcode::JumpIfFalse => {
                    let param1 = get_position(&self.codes, i + 1, opcode_value.mode_1);
                    let param2 = get_position(&self.codes, i + 2, opcode_value.mode_2) as usize;
                    if param1 == 0 {
                        self.instruction_pointer = param2;
                    } else {
                        self.instruction_pointer += 3;
                    }
                }
                Opcode::LessThan => {
                    let param1 = get_position(&self.codes, i + 1, opcode_value.mode_1);
                    let param2 = get_position(&self.codes, i + 2, opcode_value.mode_2);
                    let output_index = self.codes[i + 3] as usize;
                    self.codes[output_index] = if param1 < param2 { 1 } else { 0 };
                    self.instruction_pointer += 4;
                }
                Opcode::Equals => {
                    let param1 = get_position(&self.codes, i + 1, opcode_value.mode_1);
                    let param2 = get_position(&self.codes, i + 2, opcode_value.mode_2);
                    let output_index = self.codes[i + 3] as usize;
                    self.codes[output_index] = if param1 == param2 { 1 } else { 0 };
                    self.instruction_pointer += 4;
                }
                Opcode::Halt => {
                    self.halted = true;
                    break;
                }
                Opcode::Error(opcode_number) => {
                    panic!("Invalid opcode: {}", opcode_number);
                }
            }
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the input.");

    let original_codes: Vec<i32> = input
        .trim()
        .split(",")
        .map(|code| code.parse::<i32>().unwrap())
        .collect();

    let mut highest_signal: Option<i32> = None;

    let mut phases: [i32; 5] = [5, 6, 7, 8, 9];
    heap_recursive(&mut phases, |phase_sequence| {
        let mut amps: [Computer; 5] = [
            Computer::initialize(&original_codes),
            Computer::initialize(&original_codes),
            Computer::initialize(&original_codes),
            Computer::initialize(&original_codes),
            Computer::initialize(&original_codes),
        ];

        let mut amp_outputs: [i32; 5] = [0; 5];

        amps.iter_mut().enumerate().for_each(|(i, amp)| {
            amp.execute();
            amp.provide_input(phase_sequence[i]);
            amp.execute();
        });

        'halt_loop: loop {
            for i in 0..5 {
                amps[i].execute();
                if amps[i].halted {
                    break 'halt_loop;
                }
                amps[i].provide_input(amp_outputs[if i == 0 { 4 } else { i - 1 }]);
                amps[i].execute();
                amp_outputs[i] = amps[i].get_output();
            }
        }

        highest_signal = match highest_signal {
            None => Some(amp_outputs[4]),
            Some(signal) => {
                if signal < amp_outputs[4] {
                    Some(amp_outputs[4])
                } else {
                    highest_signal
                }
            }
        }
    });

    println!("{:?}", highest_signal);
}
