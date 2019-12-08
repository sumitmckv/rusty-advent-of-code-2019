use crate::lib::intcode_computer;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

pub fn day5a() -> std::io::Result<()> {
    let input_file = PathBuf::from("./src/day5/input.txt").canonicalize()?;
    let file = File::open(input_file)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let instructions: Vec<isize> = contents
        .split(",")
        .map(|num| num.parse().unwrap())
        .collect();
    let diagnostic_output = intcode_computer(instructions, 5, 1);
    println!("Result of day5a is {}", diagnostic_output);
    Ok(())
}

pub fn day5b() -> std::io::Result<()> {
    let input_file = PathBuf::from("./src/day5/input.txt").canonicalize()?;
    let file = File::open(input_file)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let instructions: Vec<isize> = contents
        .split(",")
        .map(|num| num.parse().unwrap())
        .collect();
    let diagnostic_output = intcode_computer(instructions, 5, 5);
    println!("Result of day5b is {}", diagnostic_output);
    Ok(())
}
