use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::PathBuf;
use std::collections::HashMap;

pub fn day3() -> std::io::Result<()>  {
    let input_file = PathBuf::from("./src/day3/input.txt").canonicalize()?;
    let file = File::open(input_file)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let lines: Vec<String> = contents.split("\r\n").map(|s| s.to_string()).collect();
    let wire1_path: Vec<String> = lines[0].split(",").map(|s| s.to_string()).collect();
    let wire2_path: Vec<String> = lines[1].split(",").map(|s| s.to_string()).collect();
    let wire1_visited = visited_map(wire1_path);
    let wire2_visited = visited_map(wire2_path);
    let mut distance = std::isize::MAX;
    let mut min_steps = std::usize::MAX;
    for (points, num_steps1) in wire1_visited.iter() {
        match wire2_visited.get(points) {
            Some(num_steps2) => {
                let (x,y) = points;
                let sum = x.abs() + y.abs();
                let step_sum = num_steps1 + num_steps2;
                distance = if sum < distance {sum} else {distance};
                min_steps = if step_sum < min_steps {step_sum} else {min_steps};
            },
            None => {}
        }
    }
    println!("Result of day3a is {}", distance);
    println!("Result of day3b is {}", min_steps);
    Ok(())
}

fn visited_map(paths: Vec<String>) -> HashMap<(isize, isize), usize> {
    let mut visited: HashMap<(isize, isize), usize> = HashMap::new();
    let mut last_x: isize = 0;
    let mut last_y: isize = 0;
    let mut num_steps = 0;
    for path in paths {
        let direction = path.chars().next().unwrap();
        let steps: usize = String::from(&path[1..]).parse().expect("Expecting number!!");

        match direction {
            'R' => {
                for _ in 0..steps {
                    last_x += 1;
                    num_steps += 1;
                    visited.insert((last_x, last_y), num_steps);
                }
            },
            'L' => {
                for _ in 0..steps {
                    last_x -= 1;
                    num_steps += 1;
                    visited.insert((last_x, last_y), num_steps);
                }
            },
            'U' => {
                for _ in 0..steps {
                    last_y += 1;
                    num_steps += 1;
                    visited.insert((last_x, last_y), num_steps);
                }
            },
            'D' => {
                for _ in 0..steps {
                    last_y -= 1;
                    num_steps += 1;
                    visited.insert((last_x, last_y), num_steps);
                }
            }
            _ => panic!("Wrnog input!!")
        }
    }
    visited 
}