use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub fn day8a() {
    let input_file = PathBuf::from("./src/day8/input.txt")
        .canonicalize()
        .unwrap();
    let input_txt = fs::read_to_string(input_file).unwrap();

    let image_data = input_txt
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();
    let width = 25;
    let height = 6;
    let mut layers: HashMap<usize, (usize, usize, usize)> = HashMap::new();
    let mut index = 0;
    let mut map_index = 0;

    while index < image_data.len() {
        let mut zero = 0;
        let mut one = 0;
        let mut two = 0;

        for slice in index..index + width * height {
            match image_data[slice] {
                0 => zero += 1,
                1 => one += 1,
                2 => two += 1,
                _ => {}
            }
            index += 1;
        }
        layers.insert(map_index, (zero, one, two));
        map_index += 1;
    }

    let min_zero_layer = layers.values().fold((std::usize::MAX, 0), |mut a, c| {
        let (zero, one, two) = c;
        let (current_min, _) = a;
        if zero <= &current_min {
            a = (*zero, one * two)
        }
        a
    });

    let (_, result) = min_zero_layer;
    println!("Result of day8a is {}", result);
}

pub fn day8b() {
    let input_file = PathBuf::from("./src/day8/input.txt")
        .canonicalize()
        .unwrap();
    let input_txt = fs::read_to_string(input_file).unwrap();

    let image_data = input_txt
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();
    let width = 25;
    let height = 6;
    let mut layers: Vec<Vec<usize>> = Vec::new();
    let mut index = 0;

    while index < image_data.len() {
        let mut slice_vec: Vec<usize> = Vec::new();

        for slice in index..index + width * height {
            slice_vec.push(image_data[slice]);
            index += 1;
        }
        layers.push(slice_vec);
    }

    index = 0;
    let mut result_vec: Vec<usize> = Vec::new();
    while index < width * height {
        result_vec.push(find(&layers, 0, index));
        index += 1;
    }

    println!("Result of day8b is:");
    for (i, v) in result_vec.iter().enumerate() {
        let print_val = if v == &0 { " " } else { "#" };
        if (i + 1) % width != 0 {
            print!("{}", print_val);
        } else {
            println!("{}", print_val);
        }
    }
}

fn find(layers: &Vec<Vec<usize>>, index1: usize, index2: usize) -> usize {
    if layers[index1][index2] == 0 || layers[index1][index2] == 1 {
        return layers[index1][index2];
    }
    find(layers, index1 + 1, index2)
}
