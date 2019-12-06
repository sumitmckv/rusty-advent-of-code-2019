use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::PathBuf;
use crate::lib::intcode_computer;

pub fn day5a() -> std::io::Result<()> {
    let input_file = PathBuf::from("./src/day5/input.txt").canonicalize()?;
    let file = File::open(input_file)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let diagnostic_output = intcode_computer(contents, 5, 1);
    print!("Result of day5a is {}\n", diagnostic_output);
    Ok(())
}

pub fn day5b() -> std::io::Result<()> {
    let input_file = PathBuf::from("./src/day5/input.txt").canonicalize()?;
    let file = File::open(input_file)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let diagnostic_output = intcode_computer(contents, 5, 5);
    print!("Result of day5b is {}\n", diagnostic_output);
    Ok(())
}

