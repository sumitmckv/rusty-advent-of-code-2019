use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::PathBuf;
use crate::lib::intcode_computer;

pub fn day2a() -> std::io::Result<()> {
    let input_file = PathBuf::from("./src/day2/input.txt").canonicalize()?;
    let file = File::open(input_file)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let mut instructions: Vec<isize> = contents.split(",").map(|num| num.parse().unwrap()).collect();
    instructions[1] = 12;
    instructions[2] = 2;
    let diagnostic_output = intcode_computer(instructions, 2, 1);
    println!("Result of day2a is {}", diagnostic_output);
    Ok(())
}

pub fn day2b() -> std::io::Result<()> {
    let input_file = PathBuf::from("./src/day2/input.txt").canonicalize()?;
    let file = File::open(input_file)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let input_vec: Vec<isize> = contents.split(",").map(|num| num.parse().unwrap()).collect();
    let target: isize = 19690720;
    let mut terminate: bool = false;
    for noun in 0..99 {
        for verb in 0..99 {
            let mut instructions = input_vec.clone();
            instructions[1] = noun;
            instructions[2] = verb;
            let diagnostic_output = intcode_computer(instructions, 2, 1);
            if target == diagnostic_output {
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