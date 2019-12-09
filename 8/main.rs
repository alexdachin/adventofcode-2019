use std::collections::HashMap;
use std::fs;

fn get_frequency(pixels: Vec<u32>) -> HashMap<u32, u32> {
    let mut frequency: HashMap<u32, u32> = HashMap::new();
    for pixel in pixels {
        frequency.insert(
            pixel,
            match frequency.get(&pixel) {
                Some(occurences) => occurences + 1,
                None => 1,
            },
        );
    }
    frequency
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the input.");

    let pixels: Vec<u32> = input
        .trim()
        .chars()
        .map(|digit| digit.to_digit(10).unwrap())
        .collect();

    let layers: Vec<Vec<u32>> = pixels
        .chunks(25 * 6)
        .map(|pixels| pixels.to_vec())
        .collect();

    let least_zeros: Vec<u32> = layers
        .clone()
        .into_iter()
        .min_by_key(|layer| match get_frequency(layer.to_vec()).get(&0) {
            Some(occurences) => *occurences,
            None => 0,
        })
        .unwrap();

    println!("{:?}", get_frequency(least_zeros));

    let image: Vec<u32> = layers.iter().fold(layers[0].clone(), |acc, layer| {
        let mut new_image = acc.clone();
        for i in 0..25 * 6 {
            if acc[i] == 2 {
                new_image[i] = layer[i];
            }
        }
        new_image
    });

    for i in 0..6 {
        for j in 0..25 {
            let pixel = image[25 * i + j];
            print!(
                "{}",
                match pixel {
                    1 => "#",
                    _ => " ",
                }
            );
        }
        print!("\n");
    }
}
