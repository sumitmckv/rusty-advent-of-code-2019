use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::PathBuf;

pub fn day2a() -> std::io::Result<()> {
    let input_file = PathBuf::from("./src/day2/input.txt").canonicalize()?;
    let file = File::open(input_file)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let mut vec: Vec<usize> = contents.split(",").map(|num| num.parse().unwrap()).collect();
    let mut cur_index = 0;
    vec[1] = 12;
    vec[2] = 2;
    while cur_index < vec.len() {
        let num = vec[cur_index];
        match num {
            99 => break,
            1 => {
                let val = vec[cur_index + 3];
                vec[val] = vec[vec[cur_index + 1]] + vec[vec[cur_index + 2]];
            },
            2 => {
                let val = vec[cur_index + 3];
                vec[val] = vec[vec[cur_index + 1]] * vec[vec[cur_index + 2]];
            },
            _ => println!("something went wrong")
        }
        cur_index += 4;
    }
    println!("Result of day2a is {}", vec[0]);
    Ok(())
}

pub fn day2b() -> std::io::Result<()> {
    let input_file = PathBuf::from("./src/day2/input.txt").canonicalize()?;
    let file = File::open(input_file)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let input_vec: Vec<usize> = contents.split(",").map(|num| num.parse().unwrap()).collect();
    let target: usize = 19690720;
    let mut terminate: bool = false;
    for noun in 0..99 {
        for verb in 0..99 {
            let mut vec = input_vec.clone();
            vec[1] = noun;
            vec[2] = verb;
            let mut cur_index = 0;
            while cur_index < vec.len() {
                let num = vec[cur_index];
                match num {
                    99 => break,
                    1 => {
                        let val = vec[cur_index + 3];
                        vec[val] = vec[vec[cur_index + 1]] + vec[vec[cur_index + 2]];
                    },
                    2 => {
                        let val = vec[cur_index + 3];
                        vec[val] = vec[vec[cur_index + 1]] * vec[vec[cur_index + 2]];
                    },
                    _ => println!("something went wrong")
                }
                cur_index += 4;
            }
            if target == vec[0] {
                println!("Result of day2b is {}", 100 * noun + verb);
                terminate = true;
                break
            }
        }
        if terminate == true {
            break
        } 
    }
    Ok(())
}