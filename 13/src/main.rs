use std::collections::HashMap;
use std::fs;

mod intcode;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Point {
    x: isize,
    y: isize,
}

fn print_grid(grid: &HashMap<Point, u8>) {
    let mut max_x: usize = 0;
    let mut max_y: usize = 0;
    for (point, _) in grid {
        max_x = max_x.max(point.x as usize);
        max_y = max_y.max(point.y as usize);
    }

    let mut output_vector: Vec<Vec<u8>> = vec![vec![0; max_x + 1]; max_y + 1];
    for (point, tile) in grid {
        output_vector[point.y as usize][point.x as usize] = *tile;
    }

    for i in 0..output_vector.len() {
        println!();
        for j in 0..output_vector[i].len() {
            print!(
                "{}",
                match output_vector[i][j] {
                    0 => " ",
                    1 => "█",
                    2 => "@",
                    3 => "X",
                    4 => "o",
                    _ => panic!("Don't know how to print!"),
                }
            );
        }
    }
}

fn count_tile(grid: &HashMap<Point, u8>, tile_id: u8) -> usize {
    grid.iter()
        .filter(|grid_element| grid_element.1 == &tile_id)
        .count()
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

    const BLOCK: u8 = 2;
    const PADDLE: u8 = 3;
    const BALL: u8 = 4;

    // Part 1
    let mut computer = intcode::Computer::initialize(&original_codes);

    let mut grid: HashMap<Point, u8> = HashMap::new();
    while !computer.halted {
        computer.execute();
        let x = computer.get_output() as isize;
        computer.execute();
        let y = computer.get_output() as isize;
        computer.execute();
        let tile_id = computer.get_output() as u8;
        computer.execute();

        grid.insert(Point { x, y }, tile_id);
    }

    let blocks = count_tile(&grid, BLOCK);
    println!("{} blocks", blocks);

    // Part 2
    original_codes.insert(0, 2);
    computer = intcode::Computer::initialize(&original_codes);
    grid = HashMap::new();
    let mut score: i64 = 0;
    let mut ball: Point = Point { x: 0, y: 0 };
    let mut paddle: Point = Point { x: 0, y: 0 };

    while !computer.halted {
        computer.execute();

        if computer.waiting_for_input {
            computer.provide_input(if ball.x > paddle.x {
                1
            } else if ball.x < paddle.x {
                -1
            } else {
                0
            });

            computer.execute();
        } else if computer.has_pending_output {
            let x = computer.get_output() as isize;
            computer.execute();
            let y = computer.get_output() as isize;
            computer.execute();

            if x == -1 && y == 0 {
                score = computer.get_output();
            } else {
                let tile_id = computer.get_output() as u8;
                computer.execute();

                if tile_id == PADDLE {
                    paddle = Point { x, y };
                }

                if tile_id == BALL {
                    ball = Point { x, y };
                }

                grid.insert(Point { x, y }, tile_id);
            }
        } else {
            let blocks = count_tile(&grid, BLOCK);
            if blocks == 0 {
                print_grid(&grid);
                println!();
                println!();
                println!("Game Over! Score: {}", score);
                break;
            }
        }
    }
}
