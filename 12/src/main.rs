use num::integer::lcm;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Point(i32, i32, i32);

#[derive(Debug, Clone)]
struct Velocity(i32, i32, i32);

fn compute_velocities(moons: &[Point; 4], velocities: &[Velocity; 4]) -> [Velocity; 4] {
    let mut new_velocities: [Velocity; 4] = velocities.clone();

    for (i, moon) in moons.iter().enumerate() {
        for pair in moons {
            if moon == pair {
                continue;
            }

            if moon.0 < pair.0 {
                new_velocities[i].0 += 1;
            } else if moon.0 > pair.0 {
                new_velocities[i].0 -= 1;
            }

            if moon.1 < pair.1 {
                new_velocities[i].1 += 1;
            } else if moon.1 > pair.1 {
                new_velocities[i].1 -= 1;
            }

            if moon.2 < pair.2 {
                new_velocities[i].2 += 1;
            } else if moon.2 > pair.2 {
                new_velocities[i].2 -= 1;
            }
        }
    }

    new_velocities
}

fn apply_velocities(points: &[Point; 4], velocities: &[Velocity; 4]) -> [Point; 4] {
    let mut new_points = points.clone();
    for i in 0..4 {
        new_points[i].0 += velocities[i].0;
        new_points[i].1 += velocities[i].1;
        new_points[i].2 += velocities[i].2;
    }

    new_points
}

fn compute_energy(points: &[Point; 4], velocities: &[Velocity; 4]) -> i32 {
    let mut energy: i32 = 0;
    for i in 0..4 {
        let mut pot: i32 = 0;
        pot += points[i].0.abs();
        pot += points[i].1.abs();
        pot += points[i].2.abs();

        let mut kin: i32 = 0;
        kin += velocities[i].0.abs();
        kin += velocities[i].1.abs();
        kin += velocities[i].2.abs();

        energy += pot * kin
    }

    energy
}

fn main() {
    const INITIAL_POSITIONS: [Point; 4] = [
        Point(5, 4, 4),
        Point(-11, -11, -3),
        Point(0, 7, 0),
        Point(-13, 2, 10),
    ];

    const INITIAL_VELOCITIES: [Velocity; 4] = [
        Velocity(0, 0, 0),
        Velocity(0, 0, 0),
        Velocity(0, 0, 0),
        Velocity(0, 0, 0),
    ];

    let mut current_positions = INITIAL_POSITIONS.clone();
    let mut current_velocities: [Velocity; 4] = INITIAL_VELOCITIES.clone();

    // Part 1
    let mut step = 0;
    loop {
        step += 1;
        current_velocities = compute_velocities(&current_positions, &current_velocities);
        current_positions = apply_velocities(&current_positions, &current_velocities);
        let energy = compute_energy(&current_positions, &current_velocities);

        if step == 1000 {
            println!(
                "Part 1:\nPoints: {:?}\nVelocities: {:?}\nEnergy: {}\n",
                current_positions, current_velocities, energy
            );
            break;
        }
    }

    // Part 2
    current_positions = INITIAL_POSITIONS.clone();
    current_velocities = INITIAL_VELOCITIES.clone();
    let mut periods: (usize, usize, usize) = (0, 0, 0);
    let mut found_period: (bool, bool, bool) = (false, false, false);
    let initial_x_positions: [i32; 4] = [
        current_positions[0].0,
        current_positions[1].0,
        current_positions[2].0,
        current_positions[3].0,
    ];

    let initial_y_positions: [i32; 4] = [
        current_positions[0].1,
        current_positions[1].1,
        current_positions[2].1,
        current_positions[3].1,
    ];

    let initial_z_positions: [i32; 4] = [
        current_positions[0].2,
        current_positions[1].2,
        current_positions[2].2,
        current_positions[3].2,
    ];

    loop {
        if !found_period.0 {
            periods.0 += 1;
        }

        if !found_period.1 {
            periods.1 += 1;
        }

        if !found_period.2 {
            periods.2 += 1;
        }

        current_velocities = compute_velocities(&current_positions, &current_velocities);
        current_positions = apply_velocities(&current_positions, &current_velocities);

        let current_x_positions: [i32; 4] = [
            current_positions[0].0,
            current_positions[1].0,
            current_positions[2].0,
            current_positions[3].0,
        ];

        let current_y_positions: [i32; 4] = [
            current_positions[0].1,
            current_positions[1].1,
            current_positions[2].1,
            current_positions[3].1,
        ];

        let current_z_positions: [i32; 4] = [
            current_positions[0].2,
            current_positions[1].2,
            current_positions[2].2,
            current_positions[3].2,
        ];

        if current_x_positions == initial_x_positions {
            found_period.0 = true;
        }

        if current_y_positions == initial_y_positions {
            found_period.1 = true;
        }

        if current_z_positions == initial_z_positions {
            found_period.2 = true;
        }

        if found_period.0 && found_period.1 && found_period.2 {
            println!(
                "Part 2:\nPeriods: {:?}\nlcm: {}",
                periods,
                lcm(1 + periods.0, lcm(1 + periods.1, 1 + periods.2))
            );
            break;
        }
    }
}
