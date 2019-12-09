#[macro_use]
extern crate cached;

use cached::UnboundCache;
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
enum OrbitPath {
    Distance(u32),
    NoPath,
}

cached_key! {
    TOTAL_ORBITS: UnboundCache<(String), u32> = UnboundCache::new();
    Key = { format!("{}", object) };
    fn get_number_of_orbits(orbits: HashMap<&str, &str>, object: &str) -> u32 = {
        return match orbits.get(object) {
            Some(result) => 1 + get_number_of_orbits(orbits.clone(), result),
            None => 0,
        };
    }
}

fn get_shortest_path(
    orbiting: HashMap<&str, &str>,
    orbited: HashMap<&str, Vec<&str>>,
    visited: HashMap<&str, bool>,
    current_object: &str,
    target_object: &str,
) -> OrbitPath {
    let mut new_visited = visited.clone();
    new_visited.insert(current_object, true);

    if current_object == target_object {
        return OrbitPath::Distance(0);
    }

    let mut options: Vec<&str> = Vec::new();

    match orbiting.get(current_object) {
        Some(object_orbited) => options.push(object_orbited),
        None => (),
    };

    match orbited.get(current_object) {
        Some(objects_orbiting) => options.extend(objects_orbiting),
        None => (),
    };

    let path = options
        .into_iter()
        .filter(|option| !new_visited.contains_key(option))
        .map(|option| {
            get_shortest_path(
                orbiting.clone(),
                orbited.clone(),
                new_visited.clone(),
                option,
                target_object,
            )
        })
        .fold(OrbitPath::NoPath, |path, object| match path {
            OrbitPath::NoPath => object,
            OrbitPath::Distance(d_path) => match object {
                OrbitPath::NoPath => path,
                OrbitPath::Distance(d_object) => {
                    if d_path < d_object {
                        path
                    } else {
                        object
                    }
                }
            },
        });

    match path {
        OrbitPath::Distance(d) => OrbitPath::Distance(1 + d),
        OrbitPath::NoPath => OrbitPath::NoPath,
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the input.");

    let map: Vec<&str> = input.lines().collect();
    let mut orbiting: HashMap<&str, &str> = HashMap::new();
    let mut orbited: HashMap<&str, Vec<&str>> = HashMap::new();
    for orbit in map {
        let parsed_orbit: Vec<&str> = orbit.split(')').collect();
        let object_orbited: &str = parsed_orbit[0];
        let object_orbiting: &str = parsed_orbit[1];
        orbiting.insert(object_orbiting, object_orbited);
        if orbited.contains_key(object_orbited) {
            let mut current_orbiting = orbited.get(object_orbited).unwrap().clone();
            current_orbiting.push(object_orbiting);
            orbited.insert(object_orbited, current_orbiting);
        } else {
            orbited.insert(object_orbited, [object_orbiting].to_vec());
        }
    }

    let mut number_of_orbits: u32 = 0;
    for (object_orbiting, _) in &orbiting {
        number_of_orbits += get_number_of_orbits(orbiting.clone(), object_orbiting);
    }

    println!("Number of orbits: {:?}", number_of_orbits);

    let shortest_path = get_shortest_path(
        orbiting.clone(),
        orbited.clone(),
        HashMap::new(),
        orbiting.get("YOU").unwrap(),
        orbiting.get("SAN").unwrap(),
    );

    println!("Shortest path: {:?}", shortest_path);
}
