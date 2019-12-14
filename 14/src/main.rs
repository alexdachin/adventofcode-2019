use std::collections::HashMap;
use std::fs;

fn parse_input(input: &str) -> Vec<Reaction> {
    input
        .trim()
        .lines()
        .map(|reaction| {
            let reaction_parts: Vec<&str> = reaction.split(" => ").collect();

            let input_chemicals = reaction_parts[0]
                .split(", ")
                .map(|chemical| {
                    let chemical_parts = chemical.split(" ").collect::<Vec<&str>>();
                    Chemical {
                        quantity: chemical_parts[0].parse::<usize>().unwrap(),
                        name: chemical_parts[1],
                    }
                })
                .collect();

            let output_chemical_parts = reaction_parts[1].split(" ").collect::<Vec<&str>>();
            let output_chemical = Chemical {
                quantity: output_chemical_parts[0].parse::<usize>().unwrap(),
                name: output_chemical_parts[1],
            };

            Reaction {
                input_chemicals,
                output_chemical,
            }
        })
        .collect::<Vec<Reaction>>()
}

#[derive(Debug, Clone)]
struct Chemical<'a> {
    quantity: usize,
    name: &'a str,
}

#[derive(Debug)]
struct Reaction<'a> {
    input_chemicals: Vec<Chemical<'a>>,
    output_chemical: Chemical<'a>,
}

fn get_ore_for<'a>(
    chemical: &Chemical<'a>,
    how_to_produce: &HashMap<&str, Reaction<'a>>,
    owned_chemicals: &mut Vec<Chemical<'a>>,
) -> usize {
    if chemical.quantity == 0 {
        return 0;
    }

    if chemical.name == "ORE" {
        return chemical.quantity;
    }

    let owned_chemical: Option<(usize, Chemical)> = (owned_chemicals.clone())
        .into_iter()
        .enumerate()
        .find(|(_, owned_chemical)| owned_chemical.name == chemical.name);

    let quantity_needed = chemical.quantity;
    let quantity_owned = match owned_chemical {
        None => 0,
        Some((i, chemical)) => {
            owned_chemicals.remove(i);
            chemical.quantity
        }
    };

    if quantity_needed <= quantity_owned {
        owned_chemicals.push(Chemical {
            name: chemical.name,
            quantity: quantity_owned - quantity_needed,
        });

        return 0;
    }

    let new_quantity_needed = quantity_needed - quantity_owned;
    let reaction = how_to_produce.get(chemical.name).unwrap();

    let number_of_reactions: usize =
        (new_quantity_needed as f64 / reaction.output_chemical.quantity as f64).ceil() as usize;

    let total_ores = reaction
        .input_chemicals
        .iter()
        .map(|chemical| {
            get_ore_for(
                &Chemical {
                    name: chemical.name,
                    quantity: chemical.quantity * number_of_reactions,
                },
                &how_to_produce,
                owned_chemicals,
            )
        })
        .sum();

    owned_chemicals.push(Chemical {
        name: chemical.name,
        quantity: (reaction.output_chemical.quantity * number_of_reactions) - new_quantity_needed,
    });

    total_ores
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the input.");

    let reactions = parse_input(&input);
    let mut how_to_produce: HashMap<&str, Reaction> = HashMap::new();
    for reaction in reactions {
        how_to_produce.insert(reaction.output_chemical.name, reaction);
    }

    // Part 1
    let ores_for_fuel = get_ore_for(
        &Chemical {
            quantity: 1,
            name: "FUEL",
        },
        &how_to_produce,
        &mut Vec::new(),
    );

    println!("{:#?} ores for 1 fuel.", ores_for_fuel);

    // Part 2
    const ORES: usize = 1_000_000_000_000;
    let mut min = ORES / ores_for_fuel;
    let mut max = ORES;
    while min < max {
        let mid = (min + max + 1) / 2;
        let ores_needed = get_ore_for(
            &Chemical {
                quantity: mid,
                name: "FUEL",
            },
            &how_to_produce,
            &mut Vec::new(),
        );

        if ores_needed <= ORES {
            min = mid;
        } else {
            max = mid - 1;
        }
    }

    println!("Can get {} fuel for {} ores.", min, ORES);
}
