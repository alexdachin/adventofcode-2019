#[derive(Debug)]
struct Repeat {
    digit: u8,
    count: u8,
}

fn get_digits(n: u32) -> Vec<u8> {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

fn get_repeats(digits: Vec<u8>) -> Vec<Repeat> {
    let mut last_repeat = Repeat {
        digit: digits[0],
        count: 0,
    };
    let mut repeats: Vec<Repeat> = Vec::new();
    for digit in digits {
        if last_repeat.digit == digit {
            last_repeat.count += 1;
        } else {
            repeats.push(last_repeat);
            last_repeat = Repeat { digit, count: 1 };
        }
    }
    repeats.push(last_repeat);
    repeats
}

fn main() {
    const START: u32 = 123257;
    const END: u32 = 647015;

    let count: usize = (START..END + 1)
        .map(|i| get_digits(i))
        .filter(|d| d.windows(2).all(|w| w[0] <= w[1]))
        .filter(|d| {
            get_repeats(d.clone())
                .iter()
                .any(|repeat| repeat.count == 2)
        })
        .count();

    println!("{:?}", count);
}
