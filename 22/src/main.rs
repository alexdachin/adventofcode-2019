use modinverse::modinverse;
use std::fs;

fn deal_into_new_stack(cards: &mut Vec<usize>) {
    cards.reverse();
}

fn cut(cards: &mut Vec<usize>, n: i16) {
    if n > 0 {
        let c: Vec<usize> = cards.drain(0..n as usize).collect();
        cards.extend(c);
    }

    if n < 0 {
        let c: Vec<usize> = cards
            .drain(0..(cards.len() as isize + n as isize) as usize)
            .collect();

        cards.extend(c);
    }
}

fn deal_with_increment(cards: &Vec<usize>, n: u16) -> Vec<usize> {
    let mut new_cards: Vec<usize> = cards.clone();
    let mut added = 0;
    let mut current = 0;

    loop {
        if added == cards.len() {
            break;
        }

        new_cards[current] = cards[added];

        current = current + n as usize;
        if current > cards.len() {
            current -= cards.len();
        }

        added += 1;
    }

    new_cards
}

// https://rob.co.bb/posts/2019-02-10-modular-exponentiation-in-rust/
fn modpow(mut base: i128, mut exp: i128, modulus: i128) -> i128 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base = base.rem_euclid(modulus);
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base).rem_euclid(modulus);
        }
        exp = exp >> 1;
        base = (base * base).rem_euclid(modulus)
    }
    result
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the input.");

    // Part 1
    let mut cards: Vec<usize> = (0..10007).collect();

    for line in input.lines() {
        if line == "deal into new stack" {
            deal_into_new_stack(&mut cards);
        } else if line.starts_with("cut ") {
            let n = line.split(" ").last().unwrap().parse::<i16>().unwrap();
            cut(&mut cards, n);
        } else if line.starts_with("deal with increment ") {
            let n = line.split(" ").last().unwrap().parse::<u16>().unwrap();

            cards = deal_with_increment(&cards, n);
        } else {
            panic!("Unknown shuffle!");
        }
    }

    for i in 0..cards.len() {
        if cards[i] == 2019 {
            println!("Part 1: {:?}", i);
            break;
        }
    }

    // Part 2
    let n: i128 = 119_315_717_514_047;
    let r: i128 = 101_741_582_076_661;
    let mut offset: i128 = 0;
    let mut increment: i128 = 1;

    for line in input.lines() {
        if line == "deal into new stack" {
            increment *= -1;
            increment = increment.rem_euclid(n);

            offset += increment;
            offset = offset.rem_euclid(n);
        } else if line.starts_with("cut ") {
            let m = line.split(" ").last().unwrap().parse::<i128>().unwrap();

            offset += m * increment;
            offset = offset.rem_euclid(n);
        } else if line.starts_with("deal with increment ") {
            let m = line.split(" ").last().unwrap().parse::<i128>().unwrap();

            increment *= modinverse(m, n).unwrap();
            increment = increment.rem_euclid(n);
        } else {
            panic!("Unknown shuffle!");
        }
    }

    let total_increment = modpow(increment, r, n);
    let total_offset: i128 = ((offset * (1 - total_increment)).rem_euclid(n)
        * modinverse(1 - increment, n).unwrap())
    .rem_euclid(n);

    println!(
        "Part 2: {}",
        ((total_offset + 2020 * total_increment).rem_euclid(n))
    );
}
