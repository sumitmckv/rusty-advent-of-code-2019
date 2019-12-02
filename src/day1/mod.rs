use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::PathBuf;

pub fn day11() -> std::io::Result<()> {
    let input_file = PathBuf::from("./src/day1/input.txt").canonicalize()?;
    let file = File::open(input_file)?;
    let buf_reader = BufReader::new(file);
    let inputs = buf_reader.lines();
    let mut result: u32 = 0;
    for input in inputs {
        let input = input.expect("Unable to read line");
        let input_num: u64 = input.parse().unwrap();
        result += ((input_num / 3) as u32) - 2;
    }
    println!("Result of day11 is {}", result);
    Ok(())
}

pub fn day12() -> std::io::Result<()> {
    let input_file = PathBuf::from("./src/day1/input.txt").canonicalize()?;
    let file = File::open(input_file)?;
    let buf_reader = BufReader::new(file);
    let inputs = buf_reader.lines();
    let mut result: u32 = 0;
    for input in inputs {
        let input = input.expect("Unable to read line");
        let input_num: u64 = input.parse().unwrap();
        let mut value = (input_num / 3) as u32 -2;
        result += value;
        while value > 0 {
            value = (value /3 ) as u32;
            if value > 2 {
                value -= 2;
                result += value;
            }
        }
    }
    println!("Result of day12 is {}", result);
    Ok(())
}