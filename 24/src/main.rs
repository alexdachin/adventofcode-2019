use std::collections::HashSet;

const ITERATIONS: usize = 200;
const LEVELS: usize = ITERATIONS * 2 + 3;

fn calculate_biodiversity(bugs: &[[bool; 5]; 5]) -> u32 {
    const TILE_POINTS: [[u32; 5]; 5] = [
        [1, 2, 4, 8, 16],
        [32, 64, 128, 256, 512],
        [1024, 2048, 4096, 8192, 16384],
        [32768, 65536, 131072, 262144, 524288],
        [1048576, 2097152, 4194304, 8388608, 16777216],
    ];

    let mut biodiversity: u32 = 0;
    for (i, line) in bugs.iter().enumerate() {
        for (j, column) in line.iter().enumerate() {
            if *column {
                biodiversity += TILE_POINTS[i][j];
            }
        }
    }

    biodiversity
}

fn count_bugs(bugs: &[[[bool; 5]; 5]]) -> u32 {
    let mut number_of_bugs: u32 = 0;
    for level in bugs {
        for line in level {
            for column in line {
                if *column {
                    number_of_bugs += 1;
                }
            }
        }
    }
    number_of_bugs
}

fn count_adjacent(bugs: &[[bool; 5]; 5], i: usize, j: usize) -> u8 {
    let mut adjacent_bugs: u8 = 0;
    adjacent_bugs += if i > 0 && bugs[i - 1][j] { 1 } else { 0 };
    adjacent_bugs += if i < 4 && bugs[i + 1][j] { 1 } else { 0 };
    adjacent_bugs += if j > 0 && bugs[i][j - 1] { 1 } else { 0 };
    adjacent_bugs += if j < 4 && bugs[i][j + 1] { 1 } else { 0 };
    adjacent_bugs
}

fn count_adjacent_part2(bugs: &[[[bool; 5]; 5]; LEVELS], i: usize, j: usize, l: usize) -> u8 {
    let mut adjacent_bugs: u8 = 0;

    // bottom
    if i == 1 && j == 2 {
        adjacent_bugs += (0..5).filter(|x| bugs[l + 1][0][*x]).count() as u8;
    }

    if i == 4 {
        adjacent_bugs += if bugs[l - 1][3][2] { 1 } else { 0 };
    }

    if (i != 1 || j != 2) && i != 4 {
        adjacent_bugs += if bugs[l][i + 1][j] { 1 } else { 0 };
    }

    // top
    if i == 3 && j == 2 {
        adjacent_bugs += (0..5).filter(|x| bugs[l + 1][4][*x]).count() as u8;
    }

    if i == 0 {
        adjacent_bugs += if bugs[l - 1][1][2] { 1 } else { 0 };
    }

    if (i != 3 || j != 2) && i != 0 {
        adjacent_bugs += if bugs[l][i - 1][j] { 1 } else { 0 };
    }

    // right
    if j == 1 && i == 2 {
        adjacent_bugs += (0..5).filter(|x| bugs[l + 1][*x][0]).count() as u8;
    }

    if j == 4 {
        adjacent_bugs += if bugs[l - 1][2][3] { 1 } else { 0 };
    }

    if (j != 1 || i != 2) && j != 4 {
        adjacent_bugs += if bugs[l][i][j + 1] { 1 } else { 0 };
    }

    // left
    if j == 3 && i == 2 {
        adjacent_bugs += (0..5).filter(|x| bugs[l + 1][*x][4]).count() as u8;
    }

    if j == 0 {
        adjacent_bugs += if bugs[l - 1][2][1] { 1 } else { 0 };
    }

    if (j != 3 || i != 2) && j != 0 {
        adjacent_bugs += if bugs[l][i][j - 1] { 1 } else { 0 };
    }

    adjacent_bugs
}

fn get_new_state(bugs: &[[bool; 5]; 5]) -> [[bool; 5]; 5] {
    let mut new_state: [[bool; 5]; 5] = *bugs;
    for (i, line) in bugs.iter().enumerate() {
        for (j, _column) in line.iter().enumerate() {
            let adjacent_bugs: u8 = count_adjacent(bugs, i, j);

            if bugs[i][j] && adjacent_bugs != 1 {
                new_state[i][j] = false;
            }

            if !bugs[i][j] && (adjacent_bugs == 1 || adjacent_bugs == 2) {
                new_state[i][j] = true;
            }
        }
    }

    new_state
}

fn get_new_state_part2(bugs: &[[[bool; 5]; 5]; LEVELS]) -> [[[bool; 5]; 5]; LEVELS] {
    let mut new_state: [[[bool; 5]; 5]; LEVELS] = *bugs;
    for l in 1..LEVELS - 1 {
        let level = bugs[l];
        for (i, line) in level.iter().enumerate() {
            for (j, _column) in line.iter().enumerate() {
                if i == 2 && j == 2 {
                    continue;
                }

                let adjacent_bugs: u8 = count_adjacent_part2(bugs, i, j, l);

                if bugs[l][i][j] && adjacent_bugs != 1 {
                    new_state[l][i][j] = false;
                }

                if !bugs[l][i][j] && (adjacent_bugs == 1 || adjacent_bugs == 2) {
                    new_state[l][i][j] = true;
                }
            }
        }
    }

    new_state
}

fn main() {
    const INPUT: [[bool; 5]; 5] = [
        [false, false, true, false, true],
        [true, true, true, true, true],
        [false, true, false, false, false],
        [false, false, false, true, false],
        [true, true, false, false, false],
    ];

    let mut bugs = INPUT;
    let mut biodiversities: HashSet<u32> = HashSet::new();
    loop {
        bugs = get_new_state(&bugs);
        let biodiversity = calculate_biodiversity(&bugs);
        if !biodiversities.insert(biodiversity) {
            println!("Part 1: {:?}", biodiversity);
            break;
        }
    }

    let mut bugs_part2: [[[bool; 5]; 5]; LEVELS] = [[[false; 5]; 5]; LEVELS];
    bugs_part2[ITERATIONS + 1] = INPUT;
    for _ in 0..ITERATIONS {
        bugs_part2 = get_new_state_part2(&bugs_part2);
    }

    println!("Part 2: {}", count_bugs(&bugs_part2));
}
