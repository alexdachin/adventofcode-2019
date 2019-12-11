use std::collections::HashMap;
use std::fs;
use std::isize::MAX;
use std::isize::MIN;

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
    AdjustRelativeBase,
    Halt,
    Error(usize),
}

#[derive(Debug)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
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
            9 => Opcode::AdjustRelativeBase,
            99 => Opcode::Halt,
            _ => Opcode::Error(value % 100),
        },
        mode_1: match value / 100 % 10 {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => ParameterMode::Relative,
        },
        mode_2: match value / 1000 % 10 {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => ParameterMode::Relative,
        },
        mode_3: match value / 10000 % 10 {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => ParameterMode::Relative,
        },
    }
}

#[derive(Debug)]
struct Computer {
    codes: HashMap<usize, i64>,
    instruction_pointer: usize,
    halted: bool,
    waiting_for_input: bool,
    has_pending_output: bool,
    relative_base: i64,
}

impl Computer {
    fn initialize(codes: &HashMap<usize, i64>) -> Computer {
        Computer {
            codes: codes.clone(),
            instruction_pointer: 0,
            halted: false,
            waiting_for_input: false,
            has_pending_output: false,
            relative_base: 0,
        }
    }

    fn get_opcode_value(&mut self) -> OpcodeValue {
        parse_opcode(*self.codes.get(&self.instruction_pointer).unwrap() as usize)
    }

    fn get_param(&mut self, position: usize, mode: ParameterMode) -> i64 {
        let at_position = *self.codes.get(&position).unwrap_or(&0) as usize;
        match mode {
            ParameterMode::Position => *self.codes.get(&at_position).unwrap_or(&0),
            ParameterMode::Immediate => at_position as i64,
            ParameterMode::Relative => *self
                .codes
                .get(&((self.relative_base + at_position as i64) as usize))
                .unwrap_or(&0),
        }
    }

    fn put_param(&mut self, position: usize, mode: ParameterMode, value: i64) {
        let at_position = *self.codes.get(&position).unwrap_or(&0) as usize;
        match mode {
            ParameterMode::Position => {
                self.codes.insert(at_position, value);
            }
            ParameterMode::Immediate => panic!("Cannot put in immediate mode"),
            ParameterMode::Relative => {
                self.codes
                    .insert((self.relative_base + at_position as i64) as usize, value);
            }
        }
    }

    fn provide_input(&mut self, input: i64) {
        let opcode_value = self.get_opcode_value();
        match opcode_value.opcode {
            Opcode::Input => {
                self.put_param(self.instruction_pointer + 1, opcode_value.mode_1, input);
                self.waiting_for_input = false;
                self.instruction_pointer += 2;
            }
            _ => {
                panic!("Input not expected");
            }
        }
    }

    fn get_output(&mut self) -> i64 {
        let opcode_value = self.get_opcode_value();
        match opcode_value.opcode {
            Opcode::Output => {
                let param1 = self.get_param(self.instruction_pointer + 1, opcode_value.mode_1);
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
            let opcode_value = self.get_opcode_value();

            match opcode_value.opcode {
                Opcode::Add => {
                    let param1 = self.get_param(i + 1, opcode_value.mode_1);
                    let param2 = self.get_param(i + 2, opcode_value.mode_2);
                    self.put_param(i + 3, opcode_value.mode_3, param1 + param2);
                    self.instruction_pointer += 4;
                }
                Opcode::Multiply => {
                    let param1 = self.get_param(i + 1, opcode_value.mode_1);
                    let param2 = self.get_param(i + 2, opcode_value.mode_2);
                    self.put_param(i + 3, opcode_value.mode_3, param1 * param2);
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
                    let param1 = self.get_param(i + 1, opcode_value.mode_1);
                    let param2 = self.get_param(i + 2, opcode_value.mode_2) as usize;
                    if param1 != 0 {
                        self.instruction_pointer = param2;
                    } else {
                        self.instruction_pointer += 3;
                    }
                }
                Opcode::JumpIfFalse => {
                    let param1 = self.get_param(i + 1, opcode_value.mode_1);
                    let param2 = self.get_param(i + 2, opcode_value.mode_2) as usize;
                    if param1 == 0 {
                        self.instruction_pointer = param2;
                    } else {
                        self.instruction_pointer += 3;
                    }
                }
                Opcode::LessThan => {
                    let param1 = self.get_param(i + 1, opcode_value.mode_1);
                    let param2 = self.get_param(i + 2, opcode_value.mode_2);
                    let value = if param1 < param2 { 1 } else { 0 };
                    self.put_param(i + 3, opcode_value.mode_3, value);
                    self.instruction_pointer += 4;
                }
                Opcode::Equals => {
                    let param1 = self.get_param(i + 1, opcode_value.mode_1);
                    let param2 = self.get_param(i + 2, opcode_value.mode_2);
                    let value = if param1 == param2 { 1 } else { 0 };
                    self.put_param(i + 3, opcode_value.mode_3, value);
                    self.instruction_pointer += 4;
                }
                Opcode::AdjustRelativeBase => {
                    let param1 = self.get_param(i + 1, opcode_value.mode_1);
                    self.relative_base += param1;
                    self.instruction_pointer += 2;
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

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Point {
    x: isize,
    y: isize,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
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

    let mut computer = Computer::initialize(&original_codes);

    let mut grid: HashMap<Point, u8> = HashMap::new();
    let mut current_point = Point { x: 0, y: 0 };
    let mut current_direction = Direction::Up;

    // start on white
    grid.insert(current_point.clone(), 1);

    computer.execute();
    while !computer.halted {
        computer.provide_input(match grid.get(&current_point) {
            Some(color) => *color as i64,
            None => 0,
        });

        computer.execute();
        let color_output = computer.get_output();
        grid.insert(current_point.clone(), color_output as u8);

        computer.execute();
        let turn = computer.get_output();
        let point_direction_tuple: (Point, Direction) = match turn {
            0 => match current_direction {
                Direction::Up => (
                    Point {
                        x: current_point.x - 1,
                        y: current_point.y,
                    },
                    Direction::Left,
                ),
                Direction::Down => (
                    Point {
                        x: current_point.x + 1,
                        y: current_point.y,
                    },
                    Direction::Right,
                ),
                Direction::Left => (
                    Point {
                        x: current_point.x,
                        y: current_point.y - 1,
                    },
                    Direction::Down,
                ),
                Direction::Right => (
                    Point {
                        x: current_point.x,
                        y: current_point.y + 1,
                    },
                    Direction::Up,
                ),
            },
            1 => match current_direction {
                Direction::Up => (
                    Point {
                        x: current_point.x + 1,
                        y: current_point.y,
                    },
                    Direction::Right,
                ),
                Direction::Down => (
                    Point {
                        x: current_point.x - 1,
                        y: current_point.y,
                    },
                    Direction::Left,
                ),
                Direction::Left => (
                    Point {
                        x: current_point.x,
                        y: current_point.y + 1,
                    },
                    Direction::Up,
                ),
                Direction::Right => (
                    Point {
                        x: current_point.x,
                        y: current_point.y - 1,
                    },
                    Direction::Down,
                ),
            },
            _ => panic!("Don't know where to move!"),
        };

        current_point = point_direction_tuple.0;
        current_direction = point_direction_tuple.1;

        computer.execute();
    }

    let mut min_coord = MAX;
    let mut max_coord = MIN;
    for (point, _) in &grid {
        min_coord = min_coord.min(point.x).min(point.y);
        max_coord = max_coord.max(point.x).max(point.y);
    }

    let grid_size = (max_coord - min_coord + 1) as usize;
    let mut output_vector: Vec<Vec<u8>> = vec![vec![0; grid_size]; grid_size];
    for (point, color) in &grid {
        output_vector[(point.x - min_coord) as usize][(point.y - min_coord) as usize] = *color;
    }

    for i in 0..output_vector.len() {
        println!();
        for j in 0..output_vector[i].len() {
            print!(
                "{}",
                match output_vector[i][j] {
                    0 => ' ',
                    1 => '#',
                    _ => panic!("Don't know how to paint!"),
                }
            );
        }
    }
}
