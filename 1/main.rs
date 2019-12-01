use std::fs;

fn get_fuel(mass: i32) -> i32 {
    let fuel = mass / 3 - 2;
    if fuel < 0 {
        return 0;
    }

    fuel + get_fuel(fuel)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the input.");

    let fuel: i32 = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .map(get_fuel)
        .sum();

    println!("{}", fuel);
}