use std::fs;

fn compute(original_codes: Vec<usize>, noun: usize, verb: usize) -> usize {
    let mut codes = original_codes.clone();
    codes[1] = noun;
    codes[2] = verb;

    let mut i: usize = 0;
    while codes[i] != 99 {
        let param1_index = codes[i + 1] as usize;
        let param2_index = codes[i + 2] as usize;
        let output_index = codes[i + 3] as usize;
        if codes[i] == 1 {
            codes[output_index] = codes[param1_index] + codes[param2_index];
        } else if codes[i] == 2 {
            codes[output_index] = codes[param1_index] * codes[param2_index];
        }

        i += 4
    }

    codes[0]
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the input.");

    let original_codes: Vec<usize> = input
        .trim()
        .split(",")
        .map(|code| code.parse::<usize>().unwrap())
        .collect();

    let expected_output = 19690720;

    let mut answer: usize = 0;
    'outer: for noun in 0..100 {
        for verb in 0..100 {
            if compute(original_codes.clone(), noun, verb) == expected_output {
                answer = 100 * noun + verb;
                break 'outer;
            }
        }
    }

    println!("{}", answer);
}
