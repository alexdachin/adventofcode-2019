use itertools::Itertools;
use std::cmp::Ordering;
use std::f64;
use std::fs;

#[derive(Clone, Debug)]
struct Point(isize, isize);

#[derive(Clone, Debug)]
struct AsteroidFromPosition {
    angle: f64,
    point: Point,
}

fn get_angles(asteroids: &Vec<Point>, position: &Point) -> Vec<AsteroidFromPosition> {
    let mut angles: Vec<AsteroidFromPosition> = Vec::new();

    asteroids
        .iter()
        .map(|asteroid| AsteroidFromPosition {
            angle: get_angle(asteroid, position),
            point: asteroid.clone(),
        })
        .sorted_by(|a, b| b.angle.partial_cmp(&a.angle).unwrap_or(Ordering::Equal))
        .group_by(|a| a.angle)
        .into_iter()
        .for_each(|(_, group)| {
            for item in group.sorted_by(|a, b| {
                get_distance(&a.point, &position)
                    .partial_cmp(&get_distance(&b.point, &position))
                    .unwrap_or(Ordering::Equal)
            }) {
                angles.push(item)
            }
        });

    angles
}

fn get_distance(asteroid: &Point, position: &Point) -> f64 {
    (((asteroid.0 - position.0).pow(2) + (asteroid.1 - position.1).pow(2)) as f64).sqrt()
}

fn get_angle(asteroid: &Point, position: &Point) -> f64 {
    ((asteroid.0 - position.0) as f64).atan2((asteroid.1 - position.1) as f64)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the input.");

    let mut asteroids: Vec<Point> = Vec::new();
    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            if char == '#' {
                asteroids.push(Point(j as isize, i as isize));
            }
        }
    }

    // Part 1
    let mut can_see_max: usize = 0;
    let mut position = Point(0, 0);
    for asteroid in &asteroids {
        let mut angles: Vec<f64> = get_angles(&asteroids, &asteroid)
            .iter()
            .map(|angle| angle.angle)
            .collect();

        angles.dedup();
        if can_see_max < angles.len() {
            can_see_max = angles.len();
            position = asteroid.clone();
        }
    }

    println!("{:?} can see {:?} asteroids", position, can_see_max);

    // Part 2
    let mut angles = get_angles(&asteroids, &position);
    let mut i = 0;
    let mut shot = 0;
    let mut last: AsteroidFromPosition;
    loop {
        last = angles.remove(i);
        shot += 1;
        println!("{} {:?}", shot, last);

        if shot == 200 {
            break;
        }

        while last.angle == angles[i].angle {
            i += 1;
            if i == angles.len() {
                i = 0;
            }
        }
    }
}
